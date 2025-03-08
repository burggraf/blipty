use super::VideoPlayer;
use serde::Serialize;
use std::sync::Arc;
use tauri::{Manager, Runtime, State};

#[derive(Serialize)]
pub struct StreamInfo {
    pub duration: Option<f64>,
    pub position: f64,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub resolution: Option<String>,
    pub bitrate: Option<u32>,
}

#[tauri::command]
pub async fn initialize_player<R: Runtime>(
    player: State<'_, VideoPlayer<R>>,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    // Access the inner VideoPlayer through the State wrapper
    Ok(()) // Nothing to initialize for now
}

#[tauri::command]
pub async fn play_video<R: Runtime>(
    url: String,
    player: State<'_, VideoPlayer<R>>,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    player.play(&url).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn pause_video<R: Runtime>(
    player: State<'_, VideoPlayer<R>>,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    player.pause().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn resume_video<R: Runtime>(
    player: State<'_, VideoPlayer<R>>,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    player.resume().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_video<R: Runtime>(
    player: State<'_, VideoPlayer<R>>,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    player.stop().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seek_video<R: Runtime>(
    position: f64,
    player: State<'_, Arc<VideoPlayer<R>>>,
) -> Result<(), String> {
    player.seek(position).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_volume<R: Runtime>(
    volume: f64,
    player: State<'_, Arc<VideoPlayer<R>>>,
) -> Result<(), String> {
    player.set_volume(volume).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_stream_info<R: Runtime>(
    player: State<'_, Arc<VideoPlayer<R>>>,
) -> Result<StreamInfo, String> {
    player.get_stream_info().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_player_state<R: Runtime>(
    player: State<'_, VideoPlayer<R>>,
    app_handle: tauri::AppHandle<R>,
) -> Result<String, String> {
    Ok(player.get_state().to_string())
}
