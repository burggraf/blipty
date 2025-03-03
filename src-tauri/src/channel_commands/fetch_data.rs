use rusqlite::params;
use serde_json::Value;
use std::collections::HashMap;
use tauri::{AppHandle, Runtime, State};

use crate::channel_commands::extract_channels::extract_channels;
use crate::channel_commands::fetch_api::fetch_api_data;
use crate::channel_commands::insert_categories::insert_categories;
use crate::channel_commands::insert_channels::insert_channels;
use crate::{db::DbConnection, models::Error};

#[tauri::command(rename_all = "camelCase")]
pub async fn fetch_and_populate_data<R: Runtime>(
    _app_handle: AppHandle<R>,
    db: State<'_, DbConnection>,
    playlist_id: i64,
    server_url: String,
    username: String,
    password: String,
) -> Result<(), Error> {
    let (api_data, live_categories, vod_categories, series_categories) =
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
    // Prepare live categories for insertion
    let mut live_categories_with_type: HashMap<String, (String, String, Option<i64>)> =
        HashMap::new();
    for (category_id, category_name) in live_categories {
        live_categories_with_type.insert(
            category_id.clone(),
            (category_name, "live".to_string(), None),
        );
    }

    // Prepare VOD categories for insertion
    let mut vod_categories_with_type: HashMap<String, (String, String, Option<i64>)> =
        HashMap::new();
    for (category_id, category_name) in vod_categories {
        vod_categories_with_type.insert(
            category_id.clone(),
            (category_name, "vod".to_string(), None),
        );
    }

    // Prepare series categories for insertion
    let mut series_categories_with_type: HashMap<String, (String, String, Option<i64>)> =
        HashMap::new();
    for (category_id, category_name) in series_categories {
        series_categories_with_type.insert(
            category_id.clone(),
            (category_name, "series".to_string(), None),
        );
    }

    // Combine categories
    let mut all_categories: HashMap<String, (String, String, Option<i64>)> = HashMap::new();
    all_categories.extend(live_categories_with_type);
    all_categories.extend(vod_categories_with_type);
    all_categories.extend(series_categories_with_type);

    insert_categories(db.clone(), &all_categories)?;

    // Extract live channels
    let all_channels = extract_channels(&api_data, "live".to_string());

    insert_channels(
        db.clone(),
        &all_channels,
        &all_categories,
        &server_url,
        &username,
        &password,
        playlist_id,
        &"live".to_string(),
    )?;

    // Extract vod channels
    let vod_channels = extract_channels(&api_data, "vod".to_string());

    insert_channels(
        db.clone(),
        &vod_channels,
        &all_categories,
        &server_url,
        &username,
        &password,
        playlist_id,
        &"vod".to_string(),
    )?;

    // Extract series channels
    let series_channels = extract_channels(&api_data, "series".to_string());

    insert_channels(
        db.clone(),
        &series_channels,
        &all_categories,
        &server_url,
        &username,
        &password,
        playlist_id,
        &"series".to_string(),
    )?;

    println!("Successfully fetched and populated data");

    Ok(())
}
