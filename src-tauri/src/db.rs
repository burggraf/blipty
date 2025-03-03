use rusqlite::{Connection, Result as SqliteResult};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(String),
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
        |row| row.get::<_, String>(0),
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
