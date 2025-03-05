pub mod channel_commands;
mod commands;
pub mod db;
pub mod models;
pub mod playlist_commands;

#[cfg(mobile)]
pub fn setup(_app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "custom-protocol")]
    _app_handle
        .manager()
        .register_uri_scheme("protocol", |_, _| {
            // Resolve to a local resource
        })?;
    // Initialize your resource
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            channel_commands::fetch_api::get_live_streams,
            channel_commands::fetch_api::get_vod,
            channel_commands::fetch_api::get_series,
            channel_commands::fetch_channels,
            channel_commands::get_selected_channel,
            channel_commands::set_selected_channel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
