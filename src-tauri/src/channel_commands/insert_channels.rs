use rusqlite::params;
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

use crate::{db::DbConnection, models::Error};

pub fn insert_channels(
    db: State<'_, DbConnection>,
    all_channels: &Vec<Value>,
    all_categories: &HashMap<String, (String, String, Option<i64>)>,
    server_url: &String,
    username: &String,
    password: &String,
    playlist_id: i64,
    _stream_type: &String, 
) -> Result<(), Error> {
    let mut conn = db.0.lock().unwrap();
    let tx = conn.transaction()?;

    // Continue using the same transaction for channels
    for channel in all_channels {
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

        let name = channel["name"]
            .as_str()
            .or(channel["title"].as_str())
            .unwrap_or("Unknown Channel");

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

        let mut _category_id_for_stream: Option<i64> = None;
        let category_name = match category_id {
            Some(id) => {
                println!("Looking for category ID: {}", id);
                match all_categories.get(id) {
                    Some((name, _, _)) => {
                        println!("Found category name: {}", name);
                        // Get the internal category ID from the categories table
                        let mut stmt =
                            tx.prepare("SELECT id FROM categories WHERE category_id = ?1")?;
                        let mut rows = stmt.query(params![id])?;

                        if let Some(row) = rows.next()? {
                            let internal_category_id: i64 = row.get(0)?;
                            _category_id_for_stream = Some(internal_category_id);
                        } else {
                            println!("Internal category ID not found for category_id: {}", id);
                            _category_id_for_stream = None;
                        }
                        name.clone()
                    }
                    None => {
                        println!("Category ID not found: {}", id);
                        _category_id_for_stream = None;
                        "Uncategorized".to_string()
                    }
                }
            }
            None => {
                println!("Category ID is None");
                _category_id_for_stream = None;
                "Uncategorized".to_string()
            }
        };

        // Get stream_type from the channel data
        let stream_type = channel["stream_type"].as_str().unwrap_or("unknown");

        // Insert into streams table
        let _result = if let Some(cat_id) = _category_id_for_stream {
            // Insert into streams table with category ID
            let sql = "INSERT OR IGNORE INTO streams (stream_id, name, stream_type, category_id, added) VALUES (?1, ?2, ?3, ?4, strftime('%s', 'now'))";
            println!("Executing SQL: {}", sql);
            println!(
                "Params: stream_id={}, name={}, stream_type={}, cat_id={}",
                stream_id, name, stream_type, cat_id
            );
            tx.execute(sql, params![stream_id, name, stream_type, cat_id])?
        } else {
            // Insert into streams table without category ID
            let sql = "INSERT OR IGNORE INTO streams (stream_id, name, stream_type, added) VALUES (?1, ?2, ?3, strftime('%s', 'now'))";
            println!("Executing SQL: {}", sql);
            println!(
                "Params: stream_id={}, name={}, stream_type={}",
                stream_id, name, stream_type
            );
            tx.execute(sql, params![stream_id, name, stream_type])?
        };

        // Also insert into channels table
        // Get stream_url from the JSON if available, otherwise construct it
        let stream_url = channel["stream_url"]
            .as_str()
            .or(channel["stream"].as_str())
            .map(|url| url.to_string())
            .unwrap_or_else(|| {
                format!(
                    "{}/live/{}/{}/{}.ts",
                    server_url, username, password, stream_id
                )
            });

        let now = chrono::Utc::now().to_rfc3339();
        let _result = tx.execute(
            "INSERT OR REPLACE INTO channels (playlist_id, category_id, category_name, stream_id, name, stream_type, stream_url, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![playlist_id, category_id, category_name, stream_id, name, stream_type, stream_url, now],
        )?;
        println!("Successfully inserted channel: {}", name);
    }

    // Commit the transaction to save the channels
    tx.commit()?;
    Ok(())
}
