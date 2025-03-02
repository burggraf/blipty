// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::async_runtime;
use tauri::path;
use tauri::{AppHandle, Manager};
mod commands;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let bundle_identifier = "net.blipty.app";

            // Initialize the database connection here
            let app_data_dir = app.handle().path().app_data_dir()
                .expect("Failed to get app data dir")
                .join(bundle_identifier);

            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
            let db_path = app_data_dir.join("iptv.db");
            println!("Database path: {:?}", db_path);
            //println!("App Config: {:?}", app.config());
            let conn = Connection::open(db_path).expect("Failed to open database connection");

            // Initialize the database schema
            commands::init_db(&conn).expect("Failed to initialize database schema");
            // Check and create the channels table if it doesn't exist
            commands::check_and_create_channels_table(&conn).expect("Failed to check/create channels table");
            app.manage(commands::DbConnection(Mutex::new(conn))); // Add the connection to the managed state

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_playlist,
            commands::get_playlists,
            commands::delete_playlist,
            commands::update_playlist,
            commands::fetch_channels,
            commands::get_selected_channel,
            commands::set_selected_channel,
            commands::get_categories,
            commands::fetch_and_populate_data,
            commands::process_m3u_content
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
