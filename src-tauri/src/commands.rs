use chrono;
use reqwest;
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Mutex;
use tauri::{AppHandle, Runtime, State};

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
    pub num: Option<String>,
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
    pub content_type: Option<String>,
    pub authenticated_stream_url: Option<String>,
    pub is_selected: Option<i64>,
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

pub fn migrate_db_v1(conn: &Connection) -> SqliteResult<()> {
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
        conn.execute("ALTER TABLE channels ADD COLUMN type_name TEXT", [])?;
    }

    // Check if category_name column exists in channels table
    if !channel_columns.contains(&"category_name".to_string()) {
        println!("Adding category_name column to channels table");
        conn.execute("ALTER TABLE channels ADD COLUMN category_name TEXT", [])?;
    }

    Ok(())
}

pub fn check_and_create_channels_table(conn: &Connection) -> SqliteResult<()> {
    // Check if the channels table exists
    let result = conn.query_row(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='channels'",
        [],
        |row| row.get::<_, String>(0)
    );
    
    if result.is_err() {
        println!("Channels table does not exist, creating it now");
        let create_channels_table = "CREATE TABLE IF NOT EXISTS channels (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            playlist_id INTEGER NOT NULL,
            category_id TEXT,
            category_name TEXT NOT NULL,
            stream_id TEXT NOT NULL,
            name TEXT NOT NULL,
            stream_type TEXT NOT NULL,
            stream_url TEXT NOT NULL,
            authenticated_stream_url TEXT,
            created_at TEXT NOT NULL,
            is_selected INTEGER DEFAULT 0,
            FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
        )";
        
        match conn.execute(create_channels_table, []) {
            Ok(_) => println!("Channels table created successfully"),
            Err(e) => {
                println!("Error creating channels table: {}", e);
                return Err(e);
            }
        };
    } else {
        println!("Channels table already exists");
    }
    
    Ok(())
}

pub fn init_db(conn: &Connection) -> SqliteResult<()> {
    let create_playlists_table = "CREATE TABLE IF NOT EXISTS playlists (
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
    )";

    let create_categories_table = "CREATE TABLE IF NOT EXISTS categories (
        id INTEGER PRIMARY KEY,
        category_id INTEGER NOT NULL UNIQUE,
        name TEXT NOT NULL,
        content_type TEXT NOT NULL DEFAULT 'live',
        type TEXT CHECK(type IN ('live', 'vod')) NOT NULL,
        parent_id INTEGER,
        created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
        updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
    )";

    let create_streams_table = "CREATE TABLE IF NOT EXISTS streams (
        id INTEGER PRIMARY KEY,
        stream_id INTEGER NOT NULL UNIQUE,
        name TEXT NOT NULL,
        category_id INTEGER REFERENCES categories(id),
        stream_type TEXT CHECK(stream_type IN ('live', 'vod', 'series')) NOT NULL,
        type_name TEXT,
        category_name TEXT,
        epg_id TEXT,
        icon_url TEXT,
        added INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
    )";

    let create_epg_data_table = "CREATE TABLE IF NOT EXISTS epg_data (
        id INTEGER PRIMARY KEY,
        channel_id TEXT NOT NULL,
        start INTEGER NOT NULL,
        end INTEGER NOT NULL,
        title TEXT NOT NULL,
        description TEXT,
        season INTEGER,
        episode INTEGER,
        FOREIGN KEY(channel_id) REFERENCES streams(epg_id)
    )";

    let create_vod_metadata_table = "CREATE TABLE IF NOT EXISTS vod_metadata (
        id INTEGER PRIMARY KEY,
        stream_id INTEGER REFERENCES streams(id),
        rating REAL,
        director TEXT,
        year INTEGER,
        plot TEXT,
        imdb_id TEXT,
        created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
        updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
    )";

    let create_selected_channel_table = "CREATE TABLE IF NOT EXISTS selected_channel (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        playlist_id INTEGER NOT NULL,
        channel_id INTEGER NOT NULL,
        created_at TEXT NOT NULL,
        UNIQUE(playlist_id),
        FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
        FOREIGN KEY(channel_id) REFERENCES streams(id) ON DELETE CASCADE
    )";

    let create_channels_table = "CREATE TABLE IF NOT EXISTS channels (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        playlist_id INTEGER NOT NULL,
        category_id TEXT,
        category_name TEXT NOT NULL,
        stream_id TEXT NOT NULL,
        name TEXT NOT NULL,
        stream_type TEXT NOT NULL,
        stream_url TEXT NOT NULL,
        authenticated_stream_url TEXT,
        created_at TEXT NOT NULL,
        is_selected INTEGER DEFAULT 0,
        FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
    )";

    match conn.execute(create_playlists_table, []) {
        Ok(_) => {}
        Err(e) => {
            println!("Error creating playlists table: {}", e);
            return Err(e);
        }
    };
    match conn.execute(create_categories_table, []) {
        Ok(_) => {}
        Err(e) => {
            println!("Error creating categories table: {}", e);
            return Err(e);
        }
    };
    match conn.execute(create_streams_table, []) {
        Ok(_) => {}
        Err(e) => {
            println!("Error creating streams table: {}", e);
            return Err(e);
        }
    };
    match conn.execute(create_epg_data_table, []) {
        Ok(_) => {}
        Err(e) => {
            println!("Error creating epg_data table: {}", e);
            return Err(e);
        }
    };
    match conn.execute(create_vod_metadata_table, []) {
        Ok(_) => {}
        Err(e) => {
            println!("Error creating vod_metadata table: {}", e);
            return Err(e);
        }
    };
    match conn.execute(create_selected_channel_table, []) {
        Ok(_) => {}
        Err(e) => {
            println!("Error creating selected_channel table: {}", e);
            return Err(e);
        }
    };
    match conn.execute(create_channels_table, []) {
        Ok(_) => {}
        Err(e) => {
            println!("Error creating channels table: {}", e);
            return Err(e);
        }
    };

    println!("Database schema initialized successfully");

    Ok(())
}

#[tauri::command]
pub async fn add_playlist(db: State<'_, DbConnection>, playlist: Playlist) -> Result<i64, Error> {
    println!("Adding playlist: {:?}", playlist);
    let conn = db.0.lock().unwrap();
    
    let result = conn.execute(
        "INSERT INTO playlists (name, server_url, username, password, epg_url, created_at, updated_at, last_updated, is_active) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![playlist.name, playlist.server_url, playlist.username, playlist.password, playlist.epg_url, playlist.created_at, playlist.updated_at, playlist.last_updated, playlist.is_active],
    );
    
    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            println!("Successfully added playlist with ID: {}", id);
            Ok(id)
        },
        Err(e) => {
            println!("Error adding playlist: {:?}", e);
            Err(Error::Database(e))
        }
    }
}

#[tauri::command]
pub async fn get_playlists(db: State<'_, DbConnection>) -> Result<Vec<Playlist>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM playlists")?;
    let playlists = stmt
        .query_map([], |row| {
            Ok(Playlist {
                id: row.get(0)?,
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
        .collect::<Result<Vec<_>, _>>()?;

    Ok(playlists)
}

#[tauri::command]
pub async fn delete_playlist(db: State<'_, DbConnection>, id: i64) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute("DELETE FROM playlists WHERE id = ?", [id])?;
    Ok(())
}

#[tauri::command]
pub async fn update_playlist(db: State<'_, DbConnection>, playlist: Playlist) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute(
        "UPDATE playlists SET name = ?1, server_url = ?2, username = ?3, password = ?4, epg_url = ?5, updated_at = ?6, last_updated = ?7, is_active = ?8 WHERE id = ?9",
        params![playlist.name, playlist.server_url, playlist.username, playlist.password, playlist.epg_url, playlist.updated_at, playlist.last_updated, playlist.is_active, playlist.id],
    )?;
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
pub async fn fetch_channels(
    db: State<'_, DbConnection>,
    playlist_id: i64,
) -> Result<Vec<Channel>, Error> {
    println!("fetch_channels called with playlist_id: {}", playlist_id);
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM channels WHERE playlist_id = ?")?;
    let channels = stmt
        .query_map([playlist_id], |row| {
            Ok(Channel {
                id: row.get(0)?,
                playlist_id: row.get(1)?,
                category_id: row.get(2)?,
                category_name: row.get(3)?,
                stream_id: row.get(4)?,
                name: row.get(5)?,
                stream_type: row.get(6)?,
                stream_url: row.get(7)?,
                authenticated_stream_url: row.get(8)?,
                created_at: row.get(9)?,
                is_selected: row.get(10)?,
                // Set default values for other fields that aren't in the database
                type_name: None,
                stream_icon: None,
                epg_channel_id: None,
                added: None,
                series_no: None,
                live: None,
                container_extension: None,
                custom_sid: None,
                tv_archive: None,
                direct_source: None,
                tv_archive_duration: None,
                num: None,
                plot: None,
                cast: None,
                director: None,
                genre: None,
                release_date: None,
                rating: None,
                rating_5based: None,
                backdrop_path: None,
                youtube_trailer: None,
                episode_run_time: None,
                cover: None,
                content_type: None,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(channels)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_selected_channel(db: State<'_, DbConnection>) -> Result<Option<Channel>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM channels WHERE is_selected = 1 LIMIT 1")?;
    let channel = stmt
        .query_map([], |row| {
            Ok(Channel {
                id: row.get(0)?,
                playlist_id: row.get(1)?,
                category_id: row.get(2)?,
                category_name: row.get(3)?,
                stream_id: row.get(4)?,
                name: row.get(5)?,
                stream_type: row.get(6)?,
                stream_url: row.get(7)?,
                authenticated_stream_url: row.get(8)?,
                created_at: row.get(9)?,
                is_selected: row.get(10)?,
                // Set default values for other fields that aren't in the database
                type_name: None,
                stream_icon: None,
                epg_channel_id: None,
                added: None,
                series_no: None,
                live: None,
                container_extension: None,
                custom_sid: None,
                tv_archive: None,
                direct_source: None,
                tv_archive_duration: None,
                num: None,
                plot: None,
                cast: None,
                director: None,
                genre: None,
                release_date: None,
                rating: None,
                rating_5based: None,
                backdrop_path: None,
                youtube_trailer: None,
                episode_run_time: None,
                cover: None,
                content_type: None,
            })
        })?
        .next()
        .transpose()?;
    Ok(channel)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn set_selected_channel(
    db: State<'_, DbConnection>,
    channel_id: i64,
) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    // First reset all selected channels
    conn.execute("UPDATE channels SET is_selected = 0", [])?;
    // Then set the new selected channel
    conn.execute(
        "UPDATE channels SET is_selected = 1 WHERE id = ?",
        [channel_id],
    )?;
    Ok(())
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_categories(
    db: State<'_, DbConnection>,
    playlist_id: i64,
) -> Result<Vec<String>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT DISTINCT category FROM channels WHERE playlist_id = ?")?;
    let categories = stmt
        .query_map([playlist_id], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(categories)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn process_m3u_content(
    db: State<'_, DbConnection>,
    playlist_id: i64,
    m3u_content: &str,
    _server_url: &str,
    _username: &str,
    _password: &str,
) -> Result<bool, Error> {
    println!("Processing M3U content...");
    
    // Parse the M3U content
    let lines: Vec<&str> = m3u_content.lines().collect();
    if lines.is_empty() || !lines[0].starts_with("#EXTM3U") {
        println!("Invalid M3U format");
        return Ok(false);
    }
    
    let mut channels = Vec::new();
    let mut current_info = String::new();
    
    for i in 1..lines.len() {
        let line = lines[i].trim();
        
        if line.starts_with("#EXTINF:") {
            current_info = line.to_string();
        } else if !line.is_empty() && !line.starts_with("#") && !current_info.is_empty() {
            // This is a URL line following an EXTINF line
            let stream_url = line.to_string();
            
            // Parse the EXTINF line to extract channel name and other metadata
            let mut channel_name = "Unknown";
            let mut category_name = "Uncategorized";
            let mut _tvg_id = "";
            
            // Extract channel name from the EXTINF line
            if let Some(name_start) = current_info.rfind(',') {
                channel_name = &current_info[name_start + 1..].trim();
            }
            
            // Extract other metadata from the EXTINF line
            let info_parts: Vec<&str> = current_info.split(' ').collect();
            for part in info_parts {
                if part.starts_with("tvg-id=\"") && part.ends_with("\"") {
                    _tvg_id = &part[8..part.len() - 1];
                } else if part.starts_with("group-title=\"") && part.ends_with("\"") {
                    category_name = &part[13..part.len() - 1];
                }
            }
            
            // Generate a unique stream ID
            let stream_id = format!("{}", channels.len() + 1);
            
            channels.push((stream_id, channel_name.to_string(), category_name.to_string(), stream_url));
            current_info = String::new();
        }
    }
    
    println!("Found {} channels in M3U content", channels.len());
    
    // Insert channels into the database
    if !channels.is_empty() {
        let mut conn = db.0.lock().unwrap();
        let tx = conn.transaction()?;
        
        for (stream_id, name, category_name, stream_url) in channels {
            let now = chrono::Utc::now().to_rfc3339();
            let result = tx.execute(
                "INSERT INTO channels (playlist_id, category_id, category_name, stream_id, name, stream_type, stream_url, created_at) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![playlist_id, None::<String>, category_name, stream_id, name, "live", stream_url, now],
            );
            
            match result {
                Ok(_) => {},
                Err(e) => {
                    println!("Error inserting channel from M3U: {}", e);
                }
            }
        }
        
        tx.commit()?;
        return Ok(true);
    }
    
    Ok(false)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn fetch_and_populate_data<R: Runtime>(
    _app_handle: AppHandle<R>,
    db: State<'_, DbConnection>,
    playlist_id: i64,
    server_url: String,
    username: String,
    password: String,
) -> Result<(), Error> {
    let mut api_data = Value::Null;
    let client = reqwest::Client::new();
    
    // Try different API endpoint formats commonly used by IPTV providers
    let endpoints = vec![
        format!("{}/api/panel_api.php?username={}&password={}", server_url, username, password),
        format!("{}/player_api.php?username={}&password={}&action=get_live_streams", server_url, username, password),
        format!("{}/player_api.php?username={}&password={}&action=get_live_categories", server_url, username, password),
        format!("{}/get.php?username={}&password={}&type=m3u_plus", server_url, username, password),
    ];
    
    let mut success = false;
    for endpoint in endpoints {
        println!("Trying API endpoint: {}", endpoint);
        match client.get(&endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Successfully connected to: {}", endpoint);
                    
                    // For M3U format, handle differently
                    if endpoint.contains("m3u_plus") {
                        let m3u_content = response.text().await?;
                        println!("Received M3U content, processing...");
                        // Process M3U content and populate channels table directly
                        // Call the process_m3u_content function directly since it's in the same module
                        success = process_m3u_content(db.clone(), playlist_id, &m3u_content, &server_url, &username, &password).await?;
                        break;
                    } else {
                        // For JSON API formats
                        match response.json::<Value>().await {
                            Ok(data) => {
                                println!("Successfully parsed JSON data");
                                // Print the top-level structure of the JSON
                                if let Some(obj) = data.as_object() {
                                    println!("JSON structure has the following top-level keys:");
                                    for (key, value) in obj {
                                        let type_str = match value {
                                            Value::Null => "null",
                                            Value::Bool(_) => "boolean",
                                            Value::Number(_) => "number",
                                            Value::String(_) => "string",
                                            Value::Array(_) => "array",
                                            Value::Object(_) => "object",
                                        };
                                        println!("  - {}: {}", key, type_str);
                                    }
                                } else {
                                    println!("JSON data is not an object, it's a: {:?}", data);
                                }
                                api_data = data;
                                success = true;
                                break;
                            }
                            Err(e) => {
                                println!("Failed to parse JSON from {}: {}", endpoint, e);
                                // Continue to next endpoint
                            }
                        }
                    }
                } else {
                    println!("Failed to connect to {}: {}", endpoint, response.status());
                }
            }
            Err(e) => {
                println!("Error connecting to {}: {}", endpoint, e);
                // Continue to next endpoint
            }
        }
    }
    
    if !success && api_data == Value::Null {
        return Err(Error::Internal("Failed to fetch data from any API endpoint".to_string()));
    }

    // Fetch categories from the API
    let mut all_categories = std::collections::HashMap::new();
    
    // Try different JSON structures for categories
    println!("Attempting to extract categories from JSON data...");
    
    // Structure 1: panel_api.php format with nested categories
    if api_data["categories"].is_object() {
        println!("Found panel_api.php style categories structure");
        if let Some(live_categories) = api_data["categories"]["live"].as_array() {
            for cat in live_categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    let parent_id = cat["parent_id"].as_i64();
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "live".to_string(), parent_id),
                    );
                }
            }
        }
        if let Some(movie_categories) = api_data["categories"]["movie"].as_array() {
            for cat in movie_categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    let parent_id = cat["parent_id"].as_i64();
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "movie".to_string(), parent_id),
                    );
                }
            }
        }
        if let Some(series_categories) = api_data["categories"]["series"].as_array() {
            for cat in series_categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    let parent_id = cat["parent_id"].as_i64();
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "series".to_string(), parent_id),
                    );
                }
            }
        }
    }
    // Structure 2: player_api.php format with direct array
    else if api_data.is_array() {
        println!("Found player_api.php style array structure");
        if let Some(categories) = api_data.as_array() {
            for cat in categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "live".to_string(), None),
                    );
                }
            }
        }
    }
    
    println!("Extracted {} categories", all_categories.len());

    // Insert categories into the database
    println!("Inserting {} categories into the database", all_categories.len());
    let mut conn = db.0.lock().unwrap();
    let tx = conn.transaction()?;
    for (cat_id, (cat_name, content_type, parent_id)) in &all_categories {
        let result = tx.execute(
            "INSERT INTO categories (category_id, name, type, parent_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, strftime('%s', 'now'), strftime('%s', 'now'))",
            params![cat_id, cat_name, content_type, parent_id],
        );
        match result {
            Ok(_) => {
                println!("Inserted category: {}", cat_name);
            }
            Err(e) => {
                println!("Error inserting category {}: {}", cat_name, e);
            }
        }
    }

    // Fetch channels from the API
    let mut all_channels = Vec::new();
    println!("Attempting to extract channels from JSON data...");
    
    // Structure 1: panel_api.php format with available_channels object
    if let Some(available_channels) = api_data["available_channels"].as_object() {
        println!("Found panel_api.php style channels structure");
        for (stream_id, channel_data) in available_channels {
            let mut channel = channel_data.clone();
            channel["stream_id"] = serde_json::Value::String(stream_id.clone());
            channel["stream_type"] = serde_json::Value::String("live".to_string());
            all_channels.push(channel);
        }
    }
    // Structure 2: player_api.php format with direct array
    else if api_data.is_array() {
        println!("Found player_api.php style array structure for channels");
        if let Some(channels) = api_data.as_array() {
            for channel in channels {
                if channel.is_object() {
                    all_channels.push(channel.clone());
                }
            }
        }
    }
    
    println!("Extracted {} channels", all_channels.len());

    // Continue using the same transaction for channels
    for channel in &all_channels {
        // Extract channel data with fallbacks for different JSON structures
        let stream_id_string: String;
        let stream_id = if let Some(id) = channel["stream_id"].as_str() {
            id
        } else if let Some(id) = channel["stream_id"].as_i64() {
            stream_id_string = id.to_string();
            &stream_id_string
        } else if let Some(id) = channel["num"].as_str() {
            id
        } else if let Some(id) = channel["num"].as_i64() {
            stream_id_string = id.to_string();
            &stream_id_string
        } else {
            "unknown"
        };
            
        let name = channel["name"].as_str()
            .or(channel["title"].as_str())
            .unwrap_or("Unknown Channel");
            
        let stream_type = channel["stream_type"].as_str()
            .unwrap_or("live");
            
        // Get category ID as a String to avoid lifetime issues
        let category_id_string: Option<String> = if let Some(id) = channel["category_id"].as_str() {
            Some(id.to_string())
        } else if let Some(id) = channel["category_id"].as_i64() {
            Some(id.to_string())
        } else {
            None
        };
        
        // Get category ID as &str for database operations
        let category_id = category_id_string.as_deref();
            
        println!("Processing channel: {} (ID: {})", name, stream_id);
            
        let category_name = category_id
            .and_then(|id| all_categories.get(id))
            .map(|(name, _, _)| name.clone())
            .unwrap_or_else(|| "Uncategorized".to_string());

        // Insert into streams table
        let result = tx.execute(
            "INSERT INTO streams (stream_id, name, stream_type, category_id, added) VALUES (?1, ?2, ?3, ?4, strftime('%s', 'now'))",
            params![stream_id, name, stream_type, category_id],
        );
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("Error inserting stream: {}", e);
            }
        }

        // Also insert into channels table
        // Get stream_url from the JSON if available, otherwise construct it
        let stream_url = channel["stream_url"].as_str()
            .or(channel["stream"].as_str())
            .map(|url| url.to_string())
            .unwrap_or_else(|| format!("{}/live/{}/{}/{}.ts", server_url, username, password, stream_id));
            
        let now = chrono::Utc::now().to_rfc3339();
        let result = tx.execute(
            "INSERT INTO channels (playlist_id, category_id, category_name, stream_id, name, stream_type, stream_url, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![playlist_id, category_id, category_name, stream_id, name, stream_type, stream_url, now],
        );
        match result {
            Ok(_) => {
                println!("Successfully inserted channel: {}", name);
            }
            Err(e) => {
                println!("Error inserting channel: {}", e);
            }
        }
    }

    // Skip VOD processing for now since we're focused on live channels
    println!("Skipping VOD processing for now");
    
    // Commit the transaction to save the channels
    let channel_count = all_channels.len();
    println!("Committing transaction to save {} channels", channel_count);
    tx.commit()?;
    println!("Transaction committed successfully");
    
    // Return success
    println!("Successfully fetched and populated data");

    Ok(())
}
