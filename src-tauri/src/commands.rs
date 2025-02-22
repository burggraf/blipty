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
pub async fn fetch_channels(id: i64, db: State<'_, DbConnection>) -> Result<String, Error> {
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

    println!("Response received: {}", body);

    Ok(body)
}
