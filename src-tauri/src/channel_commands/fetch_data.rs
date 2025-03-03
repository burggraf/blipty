use rusqlite::params;
use tauri::{AppHandle, Runtime, State};

use crate::{db::DbConnection, models::Error};

// mod fetch_api;
// mod extract_categories;
// mod extract_channels;
// mod insert_categories;
// mod insert_channels;

use crate::channel_commands::extract_categories::extract_categories;
use crate::channel_commands::extract_channels::extract_channels;
use crate::channel_commands::fetch_api::fetch_api_data;
use crate::channel_commands::insert_categories::insert_categories;
use crate::channel_commands::insert_channels::insert_channels;

use serde_json::Value;
use std::collections::HashMap;

#[tauri::command(rename_all = "camelCase")]
pub async fn fetch_and_populate_data<R: Runtime>(
    _app_handle: AppHandle<R>,
    db: State<'_, DbConnection>,
    playlist_id: i64,
    server_url: String,
    username: String,
    password: String,
) -> Result<(), Error> {
    let (api_data, categories) =
        fetch_api_data(server_url.clone(), username.clone(), password.clone()).await?;

    // For M3U format, handle differently
    if let Value::String(m3u_content) = &api_data {
        println!("Received M3U content, processing...");
        // Process M3U content and populate channels table directly
        // Call the process_m3u_content function directly since it's in the same module
        crate::channel_commands::process_m3u::process_m3u_content(
            db.clone(),
            playlist_id,
            &m3u_content,
            &server_url,
            &username,
            &password,
        )
        .await?;
        return Ok(());
    }

    // Convert the HashMap<String, String> to a HashMap<String, (String, String, Option<i64>)>
    let mut all_categories: HashMap<String, (String, String, Option<i64>)> = HashMap::new();
    for (category_id, category_name) in categories {
        all_categories.insert(
            category_id.clone(),
            (category_name, "live".to_string(), None),
        );
    }

    insert_categories(db.clone(), &all_categories)?;

    let all_channels = extract_channels(&api_data);

    insert_channels(
        db.clone(),
        &all_channels,
        &all_categories,
        &server_url,
        &username,
        &password,
        playlist_id,
    )?;

    println!("Successfully fetched and populated data");

    Ok(())
}
