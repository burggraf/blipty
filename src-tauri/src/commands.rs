use chrono::prelude::*;
use reqwest;
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, Runtime, State};
use url::Url;

// Error handling
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(String),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

pub struct DbConnection(pub Mutex<Connection>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub category_id: String,
    pub name: String,
    pub content_type: String,  // "live", "vod", or "series"
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub category_id: Option<String>,
    pub category_name: String,
    pub stream_id: String,
    pub name: String,
    pub stream_type: String,
    pub stream_url: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub server_url: String,
    pub username: String,
    pub password: String,
    pub last_updated: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSelectedChannelArgs {
    pub playlist_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSelectedChannelArgs {
    pub playlist_id: i64,
    pub channel_id: i64,
}

fn migrate_db_v1(conn: &Connection) -> SqliteResult<()> {
    // Check if content_type column exists
    let columns: Vec<String> = conn
        .prepare("PRAGMA table_info(categories)")?  
        .query_map([], |row| Ok(row.get::<_, String>(1)?))?  
        .collect::<Result<Vec<_>, _>>()?;

    if !columns.contains(&"content_type".to_string()) {
        println!("Adding content_type column to categories table");
        conn.execute(
            "ALTER TABLE categories ADD COLUMN content_type TEXT NOT NULL DEFAULT 'live'",
            [],
        )?;
    }
    Ok(())
}

pub fn init_db(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            server_url TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            last_updated TIMESTAMP,
            is_active BOOLEAN DEFAULT true,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY,
            playlist_id INTEGER NOT NULL,
            category_id TEXT NOT NULL,
            name TEXT NOT NULL,
            content_type TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
            UNIQUE(playlist_id, category_id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY,
            playlist_id INTEGER NOT NULL,
            category_id TEXT,
            stream_id TEXT NOT NULL,
            name TEXT NOT NULL,
            stream_type TEXT NOT NULL,
            stream_url TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
            UNIQUE(playlist_id, stream_id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS selected_channel (
            playlist_id INTEGER PRIMARY KEY,
            channel_id INTEGER NOT NULL,
            FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
        )",
        [],
    )?;

    println!("Database schema initialized successfully");
    
    // Run migrations
    migrate_db_v1(conn)?;
    
    Ok(())
}

#[tauri::command]
pub async fn initialize_database<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), Error> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&app_dir)?;
    let db_path = app_dir.join("iptv.db");

    let mut conn = Connection::open(db_path)?;
    init_db(&mut conn)?;

    Ok(())
}

#[tauri::command]
pub async fn add_playlist(db: State<'_, DbConnection>, playlist: Playlist) -> Result<i64, Error> {
    let mut conn = db.0.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    let mut stmt = conn.prepare(
        "INSERT INTO playlists (name, server_url, username, password, last_updated, is_active)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;

    stmt.execute(params![
        playlist.name,
        playlist.server_url,
        playlist.username,
        playlist.password,
        now,
        true
    ])?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub async fn get_playlists(db: State<'_, DbConnection>) -> Result<Vec<Playlist>, Error> {
    let mut conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, server_url, username, password, last_updated, is_active 
         FROM playlists",
    )?;

    let playlists = stmt
        .query_map([], |row| {
            Ok(Playlist {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                server_url: row.get(2)?,
                username: row.get(3)?,
                password: row.get(4)?,
                last_updated: row.get(5)?,
                is_active: row.get(6)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

    Ok(playlists)
}

#[tauri::command]
pub async fn update_playlist(
    db: State<'_, DbConnection>,
    id: i64,
    playlist: Playlist,
) -> Result<(), Error> {
    let mut conn = db.0.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    let mut stmt = conn.prepare(
        "UPDATE playlists 
         SET name = ?1, 
             server_url = ?2, 
             username = ?3, 
             password = ?4, 
             last_updated = ?5, 
             is_active = ?6
         WHERE id = ?7",
    )?;

    stmt.execute(params![
        playlist.name,
        playlist.server_url,
        playlist.username,
        playlist.password,
        now,
        playlist.is_active,
        id
    ])?;

    Ok(())
}

#[tauri::command]
pub async fn delete_playlist(db: State<'_, DbConnection>, id: i64) -> Result<(), Error> {
    let mut conn = db.0.lock().unwrap();
    conn.execute("DELETE FROM playlists WHERE id = ?", [id])?;
    Ok(())
}

#[tauri::command]
pub async fn set_selected_channel(
    db: State<'_, DbConnection>,
    args: SetSelectedChannelArgs,
) -> Result<(), Error> {
    let mut conn = db.0.lock().unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO selected_channel (playlist_id, channel_id) VALUES (?1, ?2)",
        params![args.playlist_id, args.channel_id],
    )?;
    Ok(())
}

#[tauri::command]
pub async fn get_selected_channel(
    db: State<'_, DbConnection>,
    args: GetSelectedChannelArgs,
) -> Result<Option<Channel>, Error> {
    println!("Getting selected channel for playlist: {:?}", args);
    let mut conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT c.*, COALESCE(cat.name, 'Uncategorized') as category_name
         FROM channels c
         INNER JOIN selected_channel sc ON c.id = sc.channel_id
         LEFT JOIN categories cat ON c.category_id = cat.category_id AND c.playlist_id = cat.playlist_id
         WHERE sc.playlist_id = ?",
    )?;

    let result = stmt.query_row([args.playlist_id], |row| {
        Ok(Channel {
            id: Some(row.get(0)?),
            playlist_id: row.get(1)?,
            category_id: row.get(2)?,
            stream_id: row.get(3)?,
            name: row.get(4)?,
            stream_type: row.get(5)?,
            stream_url: row.get(6)?,
            created_at: row.get(7)?,
            category_name: row.get(8)?
        })
    });

    match result {
        Ok(channel) => Ok(Some(channel)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

#[tauri::command]
pub async fn fetch_channels(id: i64, db: State<'_, DbConnection>) -> Result<Vec<Channel>, Error> {
    println!("Fetching channels for playlist: {}", id);
    {
        let mut conn = db.0.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT c.id, c.playlist_id, c.category_id, c.stream_id, c.name, c.stream_type, c.stream_url, 
                    c.created_at, cat.name as category_name, cat.content_type as category_content_type
             FROM channels c
             INNER JOIN categories cat ON c.playlist_id = cat.playlist_id AND c.category_id = cat.category_id
             WHERE c.playlist_id = ?",
        )?;

        let channels: Vec<Channel> = stmt
            .query_map([id], |row| {
                Ok(Channel {
                    id: Some(row.get(0)?),
                    playlist_id: row.get(1)?,
                    category_id: row.get(2)?,
                    category_name: row
                        .get::<_, Option<String>>(8)?
                        .unwrap_or_else(|| "Uncategorized".to_string()),
                    stream_id: row.get(3)?,
                    name: row.get(4)?,
                    stream_type: row.get(5)?,
                    stream_url: row.get(6)?,
                    created_at: row.get(7)?,
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        if !channels.is_empty() {
            return Ok(channels);
        }
    }

    let (server_url, username, password) = {
        let mut conn = db.0.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT server_url, username, password
             FROM playlists
             WHERE id = ?",
        )?;

        stmt.query_row([id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?
    };

    let mut url = Url::parse(&server_url)?;
    if !url.path().ends_with("player_api.php") {
        url.set_path("player_api.php");
    }

    let client = reqwest::Client::new();

    // First fetch categories for each content type
    let mut all_categories = std::collections::HashMap::new();
    
    // Fetch live categories
    let mut live_categories_url = url.clone();
    live_categories_url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password)
        .append_pair("action", "get_live_categories");
    
    println!("[Debug] Fetching live categories from URL: {}", live_categories_url);
    let live_categories = client.get(live_categories_url).send().await?.json::<Vec<serde_json::Value>>().await?;
    for cat in live_categories {
        if let (Some(cat_id), Some(cat_name)) = (cat["category_id"].as_str(), cat["category_name"].as_str()) {
            all_categories.insert(cat_id.to_string(), (cat_name.to_string(), "live".to_string()));
        }
    }

    // Fetch VOD categories
    let mut vod_categories_url = url.clone();
    vod_categories_url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password)
        .append_pair("action", "get_vod_categories");
    
    println!("[Debug] Fetching VOD categories from URL: {}", vod_categories_url);
    let vod_categories = client.get(vod_categories_url).send().await?.json::<Vec<serde_json::Value>>().await?;
    for cat in vod_categories {
        if let (Some(cat_id), Some(cat_name)) = (cat["category_id"].as_str(), cat["category_name"].as_str()) {
            all_categories.insert(cat_id.to_string(), (cat_name.to_string(), "vod".to_string()));
        }
    }

    // Fetch series categories
    let mut series_categories_url = url.clone();
    series_categories_url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password)
        .append_pair("action", "get_series_categories");
    
    println!("[Debug] Fetching series categories from URL: {}", series_categories_url);
    let series_categories = client.get(series_categories_url).send().await?.json::<Vec<serde_json::Value>>().await?;
    for cat in series_categories {
        if let (Some(cat_id), Some(cat_name)) = (cat["category_id"].as_str(), cat["category_name"].as_str()) {
            all_categories.insert(cat_id.to_string(), (cat_name.to_string(), "series".to_string()));
        }
    }

    println!("[Debug] Total categories found: {}", all_categories.len());
    let mut all_channels = Vec::new();

    // Fetch live streams
    let mut live_url = url.clone();
    live_url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password)
        .append_pair("action", "get_live_streams");

    println!("[Debug] Fetching live streams from URL: {}", live_url);
    let response = client.get(live_url).send().await?;
    println!("[Debug] Live streams response status: {}", response.status());
    let live_streams: Vec<serde_json::Value> = response.json().await?;
    println!("[Debug] Found {} live streams", live_streams.len());
    all_channels.extend(live_streams.into_iter().map(|mut stream| {
        stream["stream_type"] = serde_json::Value::String("live".to_string());
        stream
    }));

    // Fetch VOD streams
    let mut vod_url = url.clone();
    vod_url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password)
        .append_pair("action", "get_vod_streams");

    println!("[Debug] Fetching VOD streams from URL: {}", vod_url);
    let response = client.get(vod_url).send().await?;
    println!("[Debug] VOD streams response status: {}", response.status());
    let vod_streams: Vec<serde_json::Value> = response.json().await?;
    println!("[Debug] Found {} VOD streams", vod_streams.len());
    all_channels.extend(vod_streams.into_iter().map(|mut stream| {
        stream["stream_type"] = serde_json::Value::String("vod".to_string());
        stream
    }));

    // Fetch series streams
    let mut series_url = url.clone();
    series_url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password)
        .append_pair("action", "get_series");

    println!("[Debug] Fetching series from URL: {}", series_url);
    let response = client.get(series_url).send().await?;
    println!("[Debug] Series response status: {}", response.status());
    let series_streams: Vec<serde_json::Value> = response.json().await?;
    println!("[Debug] Found {} series", series_streams.len());
    all_channels.extend(series_streams.into_iter().map(|mut stream| {
        stream["stream_type"] = serde_json::Value::String("series".to_string());
        stream
    }));

    println!("[Debug] Total streams found: {}", all_channels.len());
    let channels = all_channels;

    // Log channel statistics and example data
    println!("[Debug] Total channels found: {}", channels.len());
    if let Some(first_channel) = channels.first() {
        println!("[Debug] Example channel data:\n{}", serde_json::to_string_pretty(first_channel).unwrap());
        
        // Log available fields in the first channel
        if let Some(obj) = first_channel.as_object() {
            println!("[Debug] Available channel fields: {}", 
                obj.keys()
                   .map(|k| k.as_str())
                   .collect::<Vec<_>>()
                   .join(", "));
        }
    }

    let get_string_value = |value: &serde_json::Value| {
        value
            .as_str()
            .map(String::from)
            .or_else(|| value.as_i64().map(|n| n.to_string()))
            .or_else(|| value.as_u64().map(|n| n.to_string()))
            .or_else(|| value.as_f64().map(|n| n.to_string()))
    };

    let mut stored_channels = Vec::new();
    let mut categories = std::collections::HashMap::new();
    let now = Utc::now().to_rfc3339();

    for channel in &channels {
        // Get category ID first
        let cat_id = get_string_value(&channel["category_id"])
            .or_else(|| get_string_value(&channel["group"]))
            .unwrap_or_else(|| "default".to_string());

        // Get category info from our fetched categories
        let (cat_name, content_type) = all_categories
            .get(&cat_id)
            .cloned()
            .unwrap_or_else(|| ("Uncategorized".to_string(), "live".to_string()));

        // Insert category with its type
        categories.entry(cat_id.clone()).or_insert_with(|| cat_name.clone());
        
        // Store the content type for later use when inserting into database
        let cat_type = content_type;
    }

    {
        let mut conn = db.0.lock().unwrap();
        let tx = conn.transaction()?;
        println!("Starting category refresh for playlist {}", id);

        // Delete all related records in the correct order
        let selected_count =
            tx.execute("DELETE FROM selected_channel WHERE playlist_id = ?", [id])?;
        println!("Deleted {} selected channel records", selected_count);

        let channels_count = tx.execute("DELETE FROM channels WHERE playlist_id = ?", [id])?;
        println!("Deleted {} channel records", channels_count);

        let categories_count = tx.execute("DELETE FROM categories WHERE playlist_id = ?", [id])?;
        println!("Deleted {} category records", categories_count);

        // Insert categories
        println!("Starting to insert {} categories", categories.len());
        for (cat_id, cat_name) in &categories {
            println!(
                "Debug: Inserting category: id={}, cat_id={}, name={}",
                id, cat_id, cat_name
            );
            let (_, content_type) = all_categories
                .get(cat_id.as_str())
                .cloned()
                .unwrap_or_else(|| ("Unknown".to_string(), "live".to_string()));

            tx.execute(
                "INSERT INTO categories (playlist_id, category_id, name, content_type, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, cat_id, cat_name, content_type, now],
            )?;
        }
        println!("Finished inserting categories");
        tx.commit()?;
    }

    {
        let mut conn = db.0.lock().unwrap();
        let tx = conn.transaction()?;
        println!("Starting to insert {} channels", channels.len());

        for channel in channels {
            let stream_id = get_string_value(&channel["stream_id"])
                .or_else(|| get_string_value(&channel["id"]))
                .or_else(|| get_string_value(&channel["num"]))
                .ok_or_else(|| Error::Internal("Missing stream_id".to_string()))?;

            let (category_id, category_name) = if let Some(cat_id) =
                get_string_value(&channel["category_id"])
                    .or_else(|| get_string_value(&channel["group"]))
            {
                (
                    Some(cat_id.clone()),
                    categories
                        .get(&cat_id)
                        .cloned()
                        .unwrap_or_else(|| "Uncategorized".to_string()),
                )
            } else {
                (None, "Uncategorized".to_string())
            };

            let name = channel["name"]
                .as_str()
                .or_else(|| channel["title"].as_str())
                .ok_or_else(|| Error::Internal("Missing name".to_string()))?
                .to_string();

            let stream_type = channel["stream_type"]
                .as_str()
                .or_else(|| channel["type"].as_str())
                .unwrap_or("live")
                .to_string();

            // Build stream URL based on content type
            let stream_url = match stream_type.as_str() {
                "live" => format!(
                    "{}/live/{}/{}/{}",
                    server_url.trim_end_matches("/player_api.php"),
                    username,
                    password,
                    stream_id
                ),
                "vod" => format!(
                    "{}/movie/{}/{}/{}",
                    server_url.trim_end_matches("/player_api.php"),
                    username,
                    password,
                    stream_id
                ),
                "series" => format!(
                    "{}/series/{}/{}/{}",
                    server_url.trim_end_matches("/player_api.php"),
                    username,
                    password,
                    stream_id
                ),
                _ => format!(
                    "{}/live/{}/{}/{}",
                    server_url.trim_end_matches("/player_api.php"),
                    username,
                    password,
                    stream_id
                )
            };

            tx.execute(
                "INSERT INTO channels (playlist_id, category_id, stream_id, name, stream_type, stream_url, created_at)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    id,
                    category_id.as_ref().map(|s| s.as_str()),
                    stream_id,
                    name,
                    stream_type,
                    stream_url,
                    now
                ],
            )?;

            stored_channels.push(Channel {
                id: Some(tx.last_insert_rowid()),
                playlist_id: id,
                category_id,
                category_name,
                stream_id,
                name,
                stream_type,
                stream_url,
                created_at: Some(now.clone()),
            });
        }

        println!("Committing channel transaction...");
        tx.commit()?;
        println!("Channel transaction committed successfully");
    }

    Ok(stored_channels)
}
