pub mod channel_commands;
mod commands;
pub mod db;
pub mod models;
pub mod playlist_commands;

#[cfg(mobile)]
// pub mod mobile;

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
