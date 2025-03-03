use rusqlite::{Connection, Result as SqliteResult};
use serde::Serialize;
use std::sync::Mutex;

pub mod migrations;

use migrations::*;

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

pub fn check_and_create_channels_table(conn: &Connection) -> SqliteResult<()> {
    migrations::create_channels_table(conn)
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

pub fn init_db(conn: &Connection) -> SqliteResult<()> {
    migrations::create_playlists_table(conn)?;
    migrations::create_categories_table(conn)?;
    migrations::create_streams_table(conn)?;
    migrations::create_epg_data_table(conn)?;
    migrations::create_vod_metadata_table(conn)?;
    migrations::create_selected_channel_table(conn)?;
    migrations::create_channels_table(conn)?;

    println!("Database schema initialized successfully");

    Ok(())
}
