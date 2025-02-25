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
    pub content_type: String,
    pub parent_id: Option<i64>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub category_id: Option<String>,
    pub stream_id: String,
    pub name: String,
    pub stream_type: String,
    pub type_name: Option<String>,
    pub stream_url: String,
    pub stream_icon: Option<String>,
    pub epg_channel_id: Option<String>,
    pub added: Option<String>,
    pub series_no: Option<String>,
    pub live: Option<String>,
    pub container_extension: Option<String>,
    pub custom_sid: Option<String>,
    pub tv_archive: Option<i64>,
    pub direct_source: Option<String>,
    pub tv_archive_duration: Option<i64>,
    pub num: Option<i64>,
    pub plot: Option<String>,
    pub cast: Option<String>,
    pub director: Option<String>,
    pub genre: Option<String>,
    pub release_date: Option<String>,
    pub rating: Option<String>,
    pub rating_5based: Option<f64>,
    pub backdrop_path: Option<Vec<String>>,
    pub youtube_trailer: Option<String>,
    pub episode_run_time: Option<String>,
    pub cover: Option<String>,
    pub created_at: Option<String>,
    pub category_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Option<i64>,
    pub name: String,
    pub server_url: String,
    pub username: String,
    pub password: String,
    pub epg_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCategoriesArgs {
    pub playlist_id: i64,
    pub content_type: Option<String>,
}

fn migrate_db_v1(conn: &Connection) -> SqliteResult<()> {
    // Check if content_type column exists in categories table
    let cat_columns: Vec<String> = conn
        .prepare("PRAGMA table_info(categories)")?  
        .query_map([], |row| Ok(row.get::<_, String>(1)?))?  
        .collect::<Result<Vec<_>, _>>()?;

    if !cat_columns.contains(&"content_type".to_string()) {
        println!("Adding content_type column to categories table");
        conn.execute(
            "ALTER TABLE categories ADD COLUMN content_type TEXT NOT NULL DEFAULT 'live'",
            [],
        )?;
    }
    
    // Check if type_name column exists in channels table
    let channel_columns: Vec<String> = conn
        .prepare("PRAGMA table_info(channels)")?  
        .query_map([], |row| Ok(row.get::<_, String>(1)?))?  
        .collect::<Result<Vec<_>, _>>()?;
        
    if !channel_columns.contains(&"type_name".to_string()) {
        println!("Adding type_name column to channels table");
        conn.execute(
            "ALTER TABLE channels ADD COLUMN type_name TEXT",
            [],
        )?;
    }
    
    // Check if category_name column exists in channels table
    if !channel_columns.contains(&"category_name".to_string()) {
        println!("Adding category_name column to channels table");
        conn.execute(
            "ALTER TABLE channels ADD COLUMN category_name TEXT",
            [],
        )?;
    }
    
    Ok(())
}

pub fn init_db(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS playlists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            server_url TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            epg_url TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT,
            last_updated TEXT,
            is_active INTEGER NOT NULL DEFAULT 1
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            playlist_id INTEGER NOT NULL,
            category_id TEXT NOT NULL,
            name TEXT NOT NULL,
            content_type TEXT NOT NULL,
            parent_id INTEGER,
            created_at TEXT NOT NULL,
            UNIQUE(playlist_id, category_id),
            FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            playlist_id INTEGER NOT NULL,
            category_id TEXT,
            stream_id TEXT NOT NULL,
            name TEXT NOT NULL,
            stream_type TEXT NOT NULL,
            type_name TEXT,
            stream_url TEXT NOT NULL,
            stream_icon TEXT,
            epg_channel_id TEXT,
            added TEXT,
            series_no TEXT,
            live TEXT,
            container_extension TEXT,
            custom_sid TEXT,
            tv_archive INTEGER,
            direct_source TEXT,
            tv_archive_duration INTEGER,
            num INTEGER,
            plot TEXT,
            cast TEXT,
            director TEXT,
            genre TEXT,
            release_date TEXT,
            rating TEXT,
            rating_5based REAL,
            backdrop_path TEXT,
            youtube_trailer TEXT,
            episode_run_time TEXT,
            cover TEXT,
            created_at TEXT NOT NULL,
            category_name TEXT,
            UNIQUE(playlist_id, stream_id),
            FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS selected_channel (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            playlist_id INTEGER NOT NULL,
            channel_id INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            UNIQUE(playlist_id),
            FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
            FOREIGN KEY(channel_id) REFERENCES channels(id) ON DELETE CASCADE
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

    let conn = Connection::open(db_path)?;
    init_db(&conn)?;

    Ok(())
}

#[tauri::command]
pub async fn add_playlist(db: State<'_, DbConnection>, playlist: Playlist) -> Result<i64, Error> {
    let conn = db.0.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    let mut stmt = conn.prepare(
        "INSERT INTO playlists (name, server_url, username, password, epg_url, created_at, updated_at, last_updated, is_active)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
    )?;

    stmt.execute(params![
        playlist.name,
        playlist.server_url,
        playlist.username,
        playlist.password,
        playlist.epg_url,
        now,
        now,
        now,
        playlist.is_active,
    ])?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub async fn get_playlists(db: State<'_, DbConnection>) -> Result<Vec<Playlist>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, server_url, username, password, epg_url, created_at, updated_at, last_updated, is_active 
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
                epg_url: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
                last_updated: row.get(8)?,
                is_active: row.get(9)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

    Ok(playlists)
}

#[tauri::command]
pub async fn update_playlist(
    id: i64,
    playlist: Playlist,
    db: State<'_, DbConnection>,
) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    let now = Utc::now().to_rfc3339();

    let mut stmt = conn.prepare(
        "UPDATE playlists 
         SET name = ?1, 
             server_url = ?2, 
             username = ?3, 
             password = ?4, 
             epg_url = ?5, 
             updated_at = ?6, 
             last_updated = ?7, 
             is_active = ?8
         WHERE id = ?9",
    )?;

    stmt.execute(params![
        playlist.name,
        playlist.server_url,
        playlist.username,
        playlist.password,
        playlist.epg_url,
        now,
        playlist.last_updated,
        playlist.is_active,
        id
    ])?;

    Ok(())
}

#[tauri::command]
pub async fn delete_playlist(db: State<'_, DbConnection>, id: i64) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute("DELETE FROM playlists WHERE id = ?", [id])?;
    Ok(())
}

#[tauri::command]
pub async fn set_selected_channel(
    db: State<'_, DbConnection>,
    args: SetSelectedChannelArgs,
) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute(
        "INSERT OR REPLACE INTO selected_channel (playlist_id, channel_id, created_at) VALUES (?1, ?2, ?3)",
        params![args.playlist_id, args.channel_id, Utc::now().to_rfc3339()],
    )?;
    Ok(())
}

#[tauri::command]
pub async fn get_selected_channel(
    db: State<'_, DbConnection>,
    args: GetSelectedChannelArgs,
) -> Result<Option<Channel>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT c.id, c.playlist_id, c.category_id, c.stream_id, c.name, c.stream_type, c.type_name, 
                c.stream_url, c.stream_icon, c.epg_channel_id, c.added, c.series_no, c.live, c.container_extension, 
                c.custom_sid, c.tv_archive, c.direct_source, c.tv_archive_duration, c.num, c.plot, 
                c.cast, c.director, c.genre, c.release_date, c.rating, c.rating_5based, c.backdrop_path, 
                c.youtube_trailer, c.episode_run_time, c.cover, c.created_at, 
                COALESCE(cat.name, c.category_name, 'Uncategorized') as category_name
         FROM channels c
         LEFT JOIN categories cat ON c.playlist_id = cat.playlist_id AND c.category_id = cat.category_id
         INNER JOIN selected_channel sc ON c.id = sc.channel_id
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
            type_name: row.get(6)?,
            stream_url: row.get(7)?,
            stream_icon: row.get(8)?,
            epg_channel_id: row.get(9)?,
            added: row.get(10)?,
            series_no: row.get(11)?,
            live: row.get(12)?,
            container_extension: row.get(13)?,
            custom_sid: row.get(14)?,
            tv_archive: row.get(15)?,
            direct_source: row.get(16)?,
            tv_archive_duration: row.get(17)?,
            num: row.get(18)?,
            plot: row.get(19)?,
            cast: row.get(20)?,
            director: row.get(21)?,
            genre: row.get(22)?,
            release_date: row.get(23)?,
            rating: row.get(24)?,
            rating_5based: row.get(25)?,
            backdrop_path: row.get::<_, Option<String>>(26)?.and_then(|s| {
                serde_json::from_str::<Vec<String>>(&s).ok()
            }),
            youtube_trailer: row.get(27)?,
            episode_run_time: row.get(28)?,
            cover: row.get(29)?,
            created_at: row.get(30)?,
            category_name: Some(row.get(31)?),
        })
    });

    match result {
        Ok(channel) => Ok(Some(channel)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(Error::Database(e)),
    }
}

#[tauri::command]
pub async fn fetch_channels(id: i64, db: State<'_, DbConnection>) -> Result<Vec<Channel>, Error> {
    println!("Fetching channels for playlist: {}", id);
    {
        let conn = db.0.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT c.id, c.playlist_id, c.category_id, c.stream_id, c.name, c.stream_type, c.type_name, 
                    c.stream_url, c.stream_icon, c.epg_channel_id, c.added, c.series_no, c.live, c.container_extension, 
                    c.custom_sid, c.tv_archive, c.direct_source, c.tv_archive_duration, c.num, c.plot, 
                    c.cast, c.director, c.genre, c.release_date, c.rating, c.rating_5based, c.backdrop_path, 
                    c.youtube_trailer, c.episode_run_time, c.cover, c.created_at, 
                    COALESCE(cat.name, c.category_name, 'Uncategorized') as category_name,
                    cat.content_type as category_content_type
             FROM channels c
             LEFT JOIN categories cat ON c.playlist_id = cat.playlist_id AND c.category_id = cat.category_id
             WHERE c.playlist_id = ?",
        )?;

        let channels = stmt
            .query_map([id], |row| {
                Ok(Channel {
                    id: Some(row.get(0)?),
                    playlist_id: row.get(1)?,
                    category_id: row.get(2)?,
                    stream_id: row.get(3)?,
                    name: row.get(4)?,
                    stream_type: row.get(5)?,
                    type_name: row.get(6)?,
                    stream_url: row.get(7)?,
                    stream_icon: row.get(8)?,
                    epg_channel_id: row.get(9)?,
                    added: row.get(10)?,
                    series_no: row.get(11)?,
                    live: row.get(12)?,
                    container_extension: row.get(13)?,
                    custom_sid: row.get(14)?,
                    tv_archive: row.get(15)?,
                    direct_source: row.get(16)?,
                    tv_archive_duration: row.get(17)?,
                    num: row.get(18)?,
                    plot: row.get(19)?,
                    cast: row.get(20)?,
                    director: row.get(21)?,
                    genre: row.get(22)?,
                    release_date: row.get(23)?,
                    rating: row.get(24)?,
                    rating_5based: row.get(25)?,
                    backdrop_path: row.get::<_, Option<String>>(26)?.and_then(|s| {
                        serde_json::from_str::<Vec<String>>(&s).ok()
                    }),
                    youtube_trailer: row.get(27)?,
                    episode_run_time: row.get(28)?,
                    cover: row.get(29)?,
                    created_at: row.get(30)?,
                    category_name: Some(row.get(31)?),
                })
            })?
            .collect::<SqliteResult<Vec<_>>>()?;

        if !channels.is_empty() {
            return Ok(channels);
        }
    }

    let (server_url, username, password) = {
        let conn = db.0.lock().unwrap();
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
    // Use panel_api.php instead of player_api.php
    if !url.path().ends_with("panel_api.php") {
        url.set_path("panel_api.php");
    }

    // Add username and password as query parameters
    url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password);

    println!("[Debug] Fetching data from URL: {}", url);
    
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    println!("[Debug] API response status: {}", response.status());
    
    let api_data: serde_json::Value = response.json().await?;
    
    // Log the structure of the response
    if let Some(obj) = api_data.as_object() {
        println!("[Debug] API response keys: {}", 
            obj.keys()
                .map(|k| k.as_str())
                .collect::<Vec<_>>()
                .join(", "));
    }

    // Extract categories from the response
    let mut all_categories = std::collections::HashMap::new();
    
    // Process live categories
    if let Some(live_categories) = api_data["categories"]["live"].as_array() {
        for cat in live_categories {
            if let (Some(cat_id), Some(cat_name)) = (cat["category_id"].as_str(), cat["category_name"].as_str()) {
                let parent_id = cat["parent_id"].as_i64();
                all_categories.insert(cat_id.to_string(), (cat_name.to_string(), "live".to_string(), parent_id));
            }
        }
    }
    
    // Process movie categories
    if let Some(movie_categories) = api_data["categories"]["movie"].as_array() {
        for cat in movie_categories {
            if let (Some(cat_id), Some(cat_name)) = (cat["category_id"].as_str(), cat["category_name"].as_str()) {
                let parent_id = cat["parent_id"].as_i64();
                all_categories.insert(cat_id.to_string(), (cat_name.to_string(), "movie".to_string(), parent_id));
            }
        }
    }
    
    // Process series categories
    if let Some(series_categories) = api_data["categories"]["series"].as_array() {
        for cat in series_categories {
            if let (Some(cat_id), Some(cat_name)) = (cat["category_id"].as_str(), cat["category_name"].as_str()) {
                let parent_id = cat["parent_id"].as_i64();
                all_categories.insert(cat_id.to_string(), (cat_name.to_string(), "series".to_string(), parent_id));
            }
        }
    }

    println!("[Debug] Total categories found: {}", all_categories.len());
    
    // Extract channels from the response
    let mut all_channels = Vec::new();
    
    // Process available channels (live streams)
    if let Some(available_channels) = api_data["available_channels"].as_object() {
        for (stream_id, channel_data) in available_channels {
            let mut channel = channel_data.clone();
            channel["stream_id"] = serde_json::Value::String(stream_id.clone());
            channel["stream_type"] = serde_json::Value::String("live".to_string());
            all_channels.push(channel);
        }
    }
    
    // Process movies
    if let Some(movies) = api_data["vod_streams"].as_object() {
        for (stream_id, movie_data) in movies {
            let mut movie = movie_data.clone();
            movie["stream_id"] = serde_json::Value::String(stream_id.clone());
            movie["stream_type"] = serde_json::Value::String("movie".to_string());
            all_channels.push(movie);
        }
    }
    
    // Process series
    if let Some(series) = api_data["series"].as_object() {
        for (stream_id, series_data) in series {
            let mut series_item = series_data.clone();
            series_item["stream_id"] = serde_json::Value::String(stream_id.clone());
            series_item["stream_type"] = serde_json::Value::String("series".to_string());
            all_channels.push(series_item);
        }
    }

    println!("[Debug] Total channels found: {}", all_channels.len());
    
    // Log channel statistics and example data
    if let Some(first_channel) = all_channels.first() {
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

    // First, store all categories
    {
        let mut conn = db.0.lock().unwrap();
        let mut tx = conn.transaction()?;
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
        println!("Starting to insert {} categories", all_categories.len());
        for (cat_id, (cat_name, content_type, parent_id)) in &all_categories {
            println!(
                "Debug: Inserting category: id={}, cat_id={}, name={}, type={}",
                id, cat_id, cat_name, content_type
            );

            tx.execute(
                "INSERT INTO categories (playlist_id, category_id, name, content_type, parent_id, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![id, cat_id, cat_name, content_type, parent_id, now],
            )?;
            
            categories.insert(cat_id.clone(), cat_name.clone());
        }
        println!("Finished inserting categories");
        tx.commit()?;
    }

    // Then, store all channels
    {
        let mut conn = db.0.lock().unwrap();
        let mut tx = conn.transaction()?;
        println!("Starting to insert {} channels", all_channels.len());

        for channel in all_channels {
            let stream_id = get_string_value(&channel["stream_id"])
                .ok_or_else(|| Error::Internal("Missing stream_id".to_string()))?;

            let (category_id, category_name) = if let Some(cat_id) =
                get_string_value(&channel["category_id"])
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
                .unwrap_or("live")
                .to_string();

            // Build stream URL based on content type
            let stream_url = match stream_type.as_str() {
                "live" => format!(
                    "{}/live/{}/{}/{}",
                    server_url.trim_end_matches("/panel_api.php"),
                    username,
                    password,
                    stream_id
                ),
                "movie" => format!(
                    "{}/movie/{}/{}/{}",
                    server_url.trim_end_matches("/panel_api.php"),
                    username,
                    password,
                    stream_id
                ),
                "series" => format!(
                    "{}/series/{}/{}/{}",
                    server_url.trim_end_matches("/panel_api.php"),
                    username,
                    password,
                    stream_id
                ),
                _ => format!(
                    "{}/live/{}/{}/{}",
                    server_url.trim_end_matches("/panel_api.php"),
                    username,
                    password,
                    stream_id
                )
            };

            // Handle backdrop_path which could be an array
            let backdrop_path_json = if let Some(arr) = channel["backdrop_path"].as_array() {
                let paths: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                if !paths.is_empty() {
                    Some(serde_json::to_string(&paths).unwrap_or_default())
                } else {
                    None
                }
            } else {
                None
            };

            tx.execute(
                "INSERT INTO channels (playlist_id, category_id, stream_id, name, stream_type, type_name, stream_url, 
                    stream_icon, epg_channel_id, added, series_no, live, container_extension, custom_sid, 
                    tv_archive, direct_source, tv_archive_duration, num, plot, cast, director, genre, 
                    release_date, rating, rating_5based, backdrop_path, youtube_trailer, episode_run_time, 
                    cover, created_at, category_name)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, 
                    ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31)",
                params![
                    id,
                    category_id.as_ref().map(|s| s.as_str()),
                    stream_id,
                    name,
                    stream_type,
                    channel["type_name"].as_str().map(String::from),
                    stream_url,
                    channel["stream_icon"].as_str().map(String::from),
                    channel["epg_channel_id"].as_str().map(String::from),
                    channel["added"].as_str().map(String::from),
                    channel["series_no"].as_str().map(String::from),
                    channel["live"].as_str().map(String::from),
                    channel["container_extension"].as_str().map(String::from),
                    channel["custom_sid"].as_str().map(String::from),
                    channel["tv_archive"].as_i64(),
                    channel["direct_source"].as_str().map(String::from),
                    channel["tv_archive_duration"].as_i64(),
                    channel["num"].as_i64(),
                    channel["plot"].as_str().map(String::from),
                    channel["cast"].as_str().map(String::from),
                    channel["director"].as_str().map(String::from),
                    channel["genre"].as_str().map(String::from),
                    channel["release_date"].as_str().map(String::from),
                    channel["rating"].as_str().map(String::from),
                    channel["rating_5based"].as_f64(),
                    backdrop_path_json,
                    channel["youtube_trailer"].as_str().map(String::from),
                    channel["episode_run_time"].as_str().map(String::from),
                    channel["cover"].as_str().map(String::from),
                    now,
                    category_name,
                ],
            )?;

            // Extract backdrop_path as Vec<String> for the Channel struct
            let backdrop_path = if let Some(arr) = channel["backdrop_path"].as_array() {
                let paths: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                if !paths.is_empty() {
                    Some(paths)
                } else {
                    None
                }
            } else {
                None
            };

            stored_channels.push(Channel {
                id: Some(tx.last_insert_rowid()),
                playlist_id: id,
                category_id,
                stream_id,
                name,
                stream_type,
                type_name: channel["type_name"].as_str().map(String::from),
                stream_url,
                stream_icon: channel["stream_icon"].as_str().map(String::from),
                epg_channel_id: channel["epg_channel_id"].as_str().map(String::from),
                added: channel["added"].as_str().map(String::from),
                series_no: channel["series_no"].as_str().map(String::from),
                live: channel["live"].as_str().map(String::from),
                container_extension: channel["container_extension"].as_str().map(String::from),
                custom_sid: channel["custom_sid"].as_str().map(String::from),
                tv_archive: channel["tv_archive"].as_i64(),
                direct_source: channel["direct_source"].as_str().map(String::from),
                tv_archive_duration: channel["tv_archive_duration"].as_i64(),
                num: channel["num"].as_i64(),
                plot: channel["plot"].as_str().map(String::from),
                cast: channel["cast"].as_str().map(String::from),
                director: channel["director"].as_str().map(String::from),
                genre: channel["genre"].as_str().map(String::from),
                release_date: channel["release_date"].as_str().map(String::from),
                rating: channel["rating"].as_str().map(String::from),
                rating_5based: channel["rating_5based"].as_f64(),
                backdrop_path,
                youtube_trailer: channel["youtube_trailer"].as_str().map(String::from),
                episode_run_time: channel["episode_run_time"].as_str().map(String::from),
                cover: channel["cover"].as_str().map(String::from),
                created_at: Some(now.clone()),
                category_name: Some(category_name),
            });
        }

        println!("Committing channel transaction...");
        tx.commit()?;
        println!("Channel transaction committed successfully");
    }

    Ok(stored_channels)
}

#[tauri::command]
pub async fn get_categories(
    db: State<'_, DbConnection>,
    args: GetCategoriesArgs,
) -> Result<Vec<Category>, Error> {
    let conn = db.0.lock().unwrap();
    
    let categories = if let Some(content_type) = &args.content_type {
        // If content_type is provided, filter by it
        let mut stmt = conn.prepare(
            "SELECT id, playlist_id, category_id, name, content_type, parent_id, created_at
             FROM categories
             WHERE playlist_id = ? AND content_type = ?
             ORDER BY name",
        )?;
        
        let mapped_rows = stmt.query_map(params![args.playlist_id, content_type], |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                playlist_id: row.get(1)?,
                category_id: row.get(2)?,
                name: row.get(3)?,
                content_type: row.get(4)?,
                parent_id: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        
        mapped_rows.collect::<SqliteResult<Vec<_>>>()?
    } else {
        // If no content_type is provided, get all categories
        let mut stmt = conn.prepare(
            "SELECT id, playlist_id, category_id, name, content_type, parent_id, created_at
             FROM categories
             WHERE playlist_id = ?
             ORDER BY name",
        )?;
        
        let mapped_rows = stmt.query_map([args.playlist_id], |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                playlist_id: row.get(1)?,
                category_id: row.get(2)?,
                name: row.get(3)?,
                content_type: row.get(4)?,
                parent_id: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        
        mapped_rows.collect::<SqliteResult<Vec<_>>>()?
    };

    Ok(categories)
}
