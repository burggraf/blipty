pub mod channel_commands;
mod commands;
pub mod db;
pub mod models;
pub mod playlist_commands;

use tauri::{AppHandle, Runtime};

#[cfg(mobile)]
// pub mod mobile;

pub fn setup<R: Runtime>(_app_handle: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "custom-protocol")]
    app_handle
        .manager()
        .register_uri_scheme("protocol", |_, _| {
            // Resolve to a local resource
        })?;
    // Initialize your resource
    Ok(())
}
