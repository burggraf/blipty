use rusqlite::{Connection, Result as SqliteResult};

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
