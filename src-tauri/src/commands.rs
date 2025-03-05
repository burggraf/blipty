use crate::db::DbConnection;
use tauri::Manager;
use tauri::State;

#[tauri::command]
pub async fn get_db_path(app_handle: tauri::AppHandle) -> Result<String, String> {
    let db_path = if cfg!(target_os = "android") {
        app_handle
            .path()
            .app_local_data_dir()
            .unwrap()
            .join("blipty.db")
    } else {
        app_handle.path().app_data_dir().unwrap().join("blipty.db")
    };
    Ok(db_path.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn initialize_database(db: State<'_, DbConnection>) -> Result<(), String> {
    Ok(()) // Database is already initialized in setup
}
