use chrono;
use reqwest;
use rusqlite::params;
use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::db::DbConnection;
use crate::models::{Channel, Error};

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
        format!(
            "{}/api/panel_api.php?username={}&password={}",
            server_url, username, password
        ),
        format!(
            "{}/player_api.php?username={}&password={}&action=get_live_streams",
            server_url, username, password
        ),
        format!(
            "{}/player_api.php?username={}&password={}&action=get_live_categories",
            server_url, username, password
        ),
        format!(
            "{}/get.php?username={}&password={}&type=m3u_plus",
            server_url, username, password
        ),
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
                        success = process_m3u_content(
                            db.clone(),
                            playlist_id,
                            &m3u_content,
                            &server_url,
                            &username,
                            &password,
                        )
                        .await?;
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
        return Err(Error::Internal(
            "Failed to fetch data from any API endpoint".to_string(),
        ));
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
    println!(
        "Inserting {} categories into the database",
        all_categories.len()
    );
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
    let channel_count = all_channels.len();
    println!("Committing transaction to save {} channels", channel_count);
    tx.commit()?;
    println!("Transaction committed successfully");

    // Return success
    println!("Successfully fetched and populated data");

    Ok(())
}
