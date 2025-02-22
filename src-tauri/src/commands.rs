use chrono::prelude::*;
use reqwest;
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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

// Custom implementation for serialization
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// Convert IO errors to our Error type
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

// Database connection wrapper
pub struct DbConnection(pub Mutex<Connection>);

// Category struct for serialization/deserialization
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub category_id: String,
    pub name: String,
    pub created_at: Option<String>,
}

// Channel struct for serialization/deserialization
#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: Option<i64>,
    pub playlist_id: i64,
    pub category_id: Option<String>,
    pub stream_id: String,
    pub name: String,
    pub stream_type: String,
    pub stream_url: String,
    pub created_at: Option<String>,
}

// Playlist struct for serialization/deserialization
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

// Initialize database
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
            FOREIGN KEY (playlist_id, category_id) REFERENCES categories(playlist_id, category_id) ON DELETE SET NULL,
            UNIQUE(playlist_id, stream_id)
        )",
        [],
    )?;

    Ok(())
}

// Command to initialize database
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

// Command to add a new playlist
#[tauri::command]
pub async fn add_playlist(db: State<'_, DbConnection>, playlist: Playlist) -> Result<i64, Error> {
    let conn = db.0.lock().unwrap();
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

// Command to get all playlists
#[tauri::command]
pub async fn get_playlists(db: State<'_, DbConnection>) -> Result<Vec<Playlist>, Error> {
    let conn = db.0.lock().unwrap();
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

// Command to update a playlist
#[tauri::command]
pub async fn update_playlist(
    db: State<'_, DbConnection>,
    id: i64,
    playlist: Playlist,
) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
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

// Command to delete a playlist
#[tauri::command]
pub async fn delete_playlist(db: State<'_, DbConnection>, id: i64) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute("DELETE FROM playlists WHERE id = ?", [id])?;
    Ok(())
}

// Command to fetch channels for a playlist
#[tauri::command]
pub async fn fetch_channels(id: i64, db: State<'_, DbConnection>) -> Result<Vec<Channel>, Error> {
    // Get playlist details from the database
    let (server_url, username, password) = {
        let conn = db.0.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT server_url, username, password
             FROM playlists
             WHERE id = ?",
        )?;

        let result = stmt.query_row([id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        result
    }; // MutexGuard is dropped here

    println!("Retrieved provider details. Building URL...");

    // Parse the server URL to handle potential port numbers
    let mut url = Url::parse(&server_url)?;

    // Ensure the path ends with player_api.php
    if !url.path().ends_with("player_api.php") {
        url.set_path("player_api.php");
    }

    // Add query parameters
    url.query_pairs_mut()
        .append_pair("username", &username)
        .append_pair("password", &password)
        .append_pair("action", "get_live_streams");

    println!("Fetching channels from URL: {}", url);

    // Make the HTTP request
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    println!("Raw response: {}", body);

    // Try to parse as a JSON Value first to inspect structure
    let json_value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| Error::Internal(format!("Failed to parse JSON: {}", e)))?;

    println!(
        "JSON structure: {}",
        serde_json::to_string_pretty(&json_value)
            .unwrap_or_else(|_| "Failed to format JSON".to_string())
    );

    // Handle different response formats
    let channels = if json_value.is_array() {
        json_value.as_array().unwrap().to_vec()
    } else if let Some(obj) = json_value.as_object() {
        // Log available keys at root level
        println!("Available root keys: {:?}", obj.keys().collect::<Vec<_>>());

        // Try common wrapping properties
        if let Some(arr) = obj.get("channels").and_then(|v| v.as_array()) {
            arr.to_vec()
        } else if let Some(arr) = obj.get("data").and_then(|v| v.as_array()) {
            arr.to_vec()
        } else if let Some(arr) = obj.get("live_streams").and_then(|v| v.as_array()) {
            arr.to_vec()
        } else {
            // If we can't find a known array property, look at each root property
            for (key, value) in obj {
                println!("Key '{}' contains: {}", key, value);
            }
            return Err(Error::Internal(format!(
                "Could not find channel array in response. Available keys: {:?}",
                obj.keys().collect::<Vec<_>>()
            )));
        }
    } else {
        return Err(Error::Internal(
            "Response is neither array nor object".to_string(),
        ));
    };

    println!("Found {} items in response", channels.len());

    let mut stored_channels = Vec::new();
    let mut stored_categories = std::collections::HashMap::new();
    let now = Utc::now().to_rfc3339();

    // First pass: collect unique categories
    let mut categories = std::collections::HashSet::new();
    for channel in &channels {
        if let (Some(cat_id), Some(cat_name)) = (
            channel["category_id"]
                .as_str()
                .or_else(|| channel["group"].as_str()),
            channel["category_name"]
                .as_str()
                .or_else(|| channel["group_title"].as_str())
                .or_else(|| channel["category_id"].as_str()),
        ) {
            categories.insert((cat_id.to_string(), cat_name.to_string()));
        }
    }

    // Process channel data
    let channel_data: Vec<(String, Option<String>, String, String, String)> = channels
        .iter()
        .map(|channel| {
            println!("Processing channel: {}", channel);

            // Helper function to get value as string from various types
            let get_string_value = |value: &serde_json::Value| {
                value
                    .as_str()
                    .map(|s| s.to_string())
                    .or_else(|| value.as_i64().map(|n| n.to_string()))
                    .or_else(|| value.as_u64().map(|n| n.to_string()))
                    .or_else(|| value.as_f64().map(|n| n.to_string()))
            };

            // Try multiple possible field names for stream_id
            let stream_id = get_string_value(&channel["stream_id"])
                .or_else(|| get_string_value(&channel["id"]))
                .or_else(|| get_string_value(&channel["num"]))
                .ok_or_else(|| Error::Internal("Missing stream_id/id/num".to_string()))?;

            // Get category ID
            let category_id = get_string_value(&channel["category_id"])
                .or_else(|| get_string_value(&channel["group"]));

            // Get channel name
            let name = channel["name"]
                .as_str()
                .or_else(|| channel["title"].as_str())
                .ok_or_else(|| Error::Internal("Missing name/title".to_string()))?
                .to_string();

            // Get stream type
            let stream_type = channel["stream_type"]
                .as_str()
                .or_else(|| channel["type"].as_str())
                .unwrap_or("live")
                .to_string();

            // For this provider, construct the stream URL using the stream_id
            let stream_url = format!(
                "{}/live/{}/{}/{}",
                server_url.trim_end_matches("/player_api.php"),
                username,
                password,
                stream_id
            );

            Ok((stream_id, category_id, name, stream_type, stream_url))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    // Store both categories and channels in a single transaction
    {
        let mut conn = db.0.lock().unwrap();
        let tx = conn.transaction()?;

        // Clear and insert categories
        tx.execute("DELETE FROM categories WHERE playlist_id = ?", [id])?;
        for (cat_id, cat_name) in &categories {
            tx.execute(
                "INSERT INTO categories (playlist_id, category_id, name, created_at) VALUES (?1, ?2, ?3, ?4)",
                params![id, cat_id, cat_name, now],
            )?;
            stored_categories.insert(cat_id.clone(), cat_name.clone());
        }

        // Clear and insert channels
        tx.execute("DELETE FROM channels WHERE playlist_id = ?", [id])?;
        for (stream_id, category_id, name, stream_type, stream_url) in &channel_data {
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

            let last_id = tx.last_insert_rowid();
            stored_channels.push(Channel {
                id: Some(last_id),
                playlist_id: id,
                category_id: category_id.clone(),
                stream_id: stream_id.clone(),
                name: name.clone(),
                stream_type: stream_type.clone(),
                stream_url: stream_url.clone(),
                created_at: Some(now.clone()),
            });
        }

        tx.commit()?;
    }

    println!("Stored {} channels in database", stored_channels.len());

    Ok(stored_channels)
}

// Command to get channels for a playlist
#[tauri::command]
pub async fn get_channels(
    db: State<'_, DbConnection>,
    playlist_id: i64,
) -> Result<Vec<Channel>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, playlist_id, category_id, stream_id, name, stream_type, stream_url, created_at
         FROM channels
         WHERE playlist_id = ?",
    )?;

    let channels = stmt
        .query_map([playlist_id], |row| {
            Ok(Channel {
                id: Some(row.get(0)?),
                playlist_id: row.get(1)?,
                category_id: row.get(2)?,
                stream_id: row.get(3)?,
                name: row.get(4)?,
                stream_type: row.get(5)?,
                stream_url: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?
        .collect::<SqliteResult<Vec<_>>>()?;

    Ok(channels)
}
