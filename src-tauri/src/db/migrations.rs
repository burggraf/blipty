use rusqlite::{Connection, Result as SqliteResult};

pub fn create_playlists_table(conn: &Connection) -> SqliteResult<()> {
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

    conn.execute(create_playlists_table, [])?;
    println!("Playlists table created successfully");
    Ok(())
}

pub fn create_categories_table(conn: &Connection) -> SqliteResult<()> {
    let create_categories_table = "CREATE TABLE IF NOT EXISTS categories (
        id INTEGER PRIMARY KEY,
        category_id INTEGER NOT NULL UNIQUE,
        name TEXT NOT NULL,
        content_type TEXT NOT NULL DEFAULT 'live',
        type TEXT CHECK(type IN ('live', 'vod', 'movie', 'series')) NOT NULL DEFAULT 'movie',
        parent_id INTEGER,
        created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
        updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
    )";

    conn.execute(create_categories_table, [])?;
    println!("Categories table created successfully");
    Ok(())
}

pub fn create_streams_table(conn: &Connection) -> SqliteResult<()> {
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

    conn.execute(create_streams_table, [])?;
    println!("Streams table created successfully");
    Ok(())
}

pub fn create_epg_data_table(conn: &Connection) -> SqliteResult<()> {
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

    conn.execute(create_epg_data_table, [])?;
    println!("Epg data table created successfully");
    Ok(())
}

pub fn create_vod_metadata_table(conn: &Connection) -> SqliteResult<()> {
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

    conn.execute(create_vod_metadata_table, [])?;
    println!("Vod metadata table created successfully");
    Ok(())
}

pub fn create_selected_channel_table(conn: &Connection) -> SqliteResult<()> {
    let create_selected_channel_table = "CREATE TABLE IF NOT EXISTS selected_channel (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        playlist_id INTEGER NOT NULL,
        channel_id INTEGER NOT NULL,
        created_at TEXT NOT NULL,
        UNIQUE(playlist_id),
        FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
        FOREIGN KEY(channel_id) REFERENCES streams(id) ON DELETE CASCADE
    )";

    conn.execute(create_selected_channel_table, [])?;
    println!("Selected channel table created successfully");
    Ok(())
}

pub fn create_channels_table(conn: &Connection) -> SqliteResult<()> {
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
            FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
        )";

        match conn.execute(create_channels_table, []) {
            Ok(_) => println!("Channels table created successfully"),
            Err(e) => {
                println!("Error creating channels table: {}", e);
                return Err(e);
            }
        };

        // Add unique index if it doesn't exist
        let create_unique_index = "CREATE UNIQUE INDEX IF NOT EXISTS idx_channels_playlist_stream ON channels (playlist_id, stream_id)";
        match conn.execute(create_unique_index, []) {
            Ok(_) => println!("Unique index on playlist_id and stream_id created successfully"),
            Err(e) => {
                println!("Error creating unique index: {}", e);
                return Err(e);
            }
        }
    } else {
        println!("Channels table already exists");
    }

    Ok(())
}

pub fn create_favorites_table(conn: &Connection) -> SqliteResult<()> {
    let create_favorites_table = "CREATE TABLE IF NOT EXISTS favorites (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        playlist_id INTEGER NOT NULL,
        stream_id TEXT NOT NULL,
        content_type TEXT NOT NULL,
        created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
        UNIQUE(playlist_id, stream_id),
        FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
    )";

    conn.execute(create_favorites_table, [])?;
    println!("Favorites table created successfully");
    Ok(())
}
