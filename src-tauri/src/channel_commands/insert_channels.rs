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

        let stream_type = channel["stream_type"].as_str().unwrap_or("live");

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
    // let channel_count = all_channels.len();
    // println!("Committing transaction to save {} channels", channel_count);
    tx.commit()?;
    // println!("Transaction committed successfully");
    Ok(())
}
