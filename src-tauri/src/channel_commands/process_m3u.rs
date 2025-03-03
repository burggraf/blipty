use chrono;
use rusqlite::params;
use tauri::State;

use crate::{db::DbConnection, models::Error};

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

            channels.push((
                stream_id,
                channel_name.to_string(),
                category_name.to_string(),
                stream_url,
            ));
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
                Ok(_) => {}
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
