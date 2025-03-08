// For Android, we use a minimal reqwest client without native TLS
#[cfg(target_os = "android")]
use reqwest::{Client, ClientBuilder};
// For other platforms, use the default reqwest
#[cfg(not(target_os = "android"))]
use reqwest;

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

use crate::{db::DbConnection, models::Error};
use crate::channel_commands::extract_channels::extract_channels;

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    _result: String,  // Prefixed with _ to indicate intentionally unused
    _data: T,        // Prefixed with _ to indicate intentionally unused
}

#[tauri::command]
#[allow(dead_code)]  // Since these functions might be used by the frontend
pub async fn get_live_streams(
    _db: State<'_, DbConnection>,
    username: String,
    password: String,
) -> Result<String, String> {
    let url = format!(
        "http://iptv.example.com/player_api.php?username={}&password={}&action=get_live_streams",
        username, password
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

#[tauri::command]
#[allow(dead_code)]
pub async fn get_vod(
    _db: State<'_, DbConnection>,
    username: String,
    password: String,
) -> Result<String, String> {
    let url = format!(
        "http://iptv.example.com/player_api.php?username={}&password={}&action=get_vod_streams",
        username, password
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}

#[tauri::command]
#[allow(dead_code)]
pub async fn get_series(
    _db: State<'_, DbConnection>,
    username: String,
    password: String,
) -> Result<String, String> {
    let url = format!(
        "http://iptv.example.com/player_api.php?username={}&password={}&action=get_series",
        username, password
    );

    let response = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;

    Ok(response)
}
