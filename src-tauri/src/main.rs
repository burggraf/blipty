// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(mobile, feature(proc_macro_hygiene))]

use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{Manager, Runtime};

mod channel_commands;
mod commands;
mod db;
mod models;
mod playlist_commands;

#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn android_main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Initialize the database connection here
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
            let db_path = app_data_dir.join("blipty.db");
            println!("Database path: {:?}", db_path);

            let conn = Connection::open(db_path).expect("Failed to open database connection");

            // Initialize the database schema
            db::init_db(&conn).expect("Failed to initialize database schema");
            db::check_and_create_channels_table(&conn)
                .expect("Failed to check/create channels table");
            db::migrate_db_v1(&conn).expect("Failed to migrate database");
            app.manage(db::DbConnection(Mutex::new(conn)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            playlist_commands::add_playlist,
            playlist_commands::get_playlists,
            playlist_commands::delete_playlist,
            playlist_commands::update_playlist,
            channel_commands::fetch_channels,
            channel_commands::get_selected_channel,
            channel_commands::set_selected_channel,
            channel_commands::category_commands::get_categories,
            channel_commands::import_commands::fetch_and_populate_data,
            channel_commands::import_commands::process_m3u_content
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

#[cfg(not(mobile))]
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Initialize the database connection here
            let app_data_dir = app_handle
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");
            let db_path = app_data_dir.join("blipty.db");
            println!("Database path: {:?}", db_path);

            let conn = Connection::open(db_path).expect("Failed to open database connection");

            // Initialize the database schema
            db::init_db(&conn).expect("Failed to initialize database schema");
            db::check_and_create_channels_table(&conn)
                .expect("Failed to check/create channels table");
            db::migrate_db_v1(&conn).expect("Failed to migrate database");
            app.manage(db::DbConnection(Mutex::new(conn)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            playlist_commands::add_playlist,
            playlist_commands::get_playlists,
            playlist_commands::delete_playlist,
            playlist_commands::update_playlist,
            channel_commands::fetch_channels,
            channel_commands::get_selected_channel,
            channel_commands::set_selected_channel,
            channel_commands::category_commands::get_categories,
            channel_commands::import_commands::fetch_and_populate_data,
            channel_commands::import_commands::process_m3u_content
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
