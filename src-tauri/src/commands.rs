use crate::db::DbConnection;
use crate::models::Error;
use rusqlite::params;
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
pub async fn initialize_database(_db: State<'_, DbConnection>) -> Result<(), String> {
    Ok(()) // Database is already initialized in setup
}

#[tauri::command]
pub async fn add_to_favorites(
    db: State<'_, DbConnection>,
    playlist_id: i64,
    stream_id: String,
    content_type: String,
) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute(
        "INSERT INTO favorites (playlist_id, stream_id, content_type) VALUES (?1, ?2, ?3)",
        params![playlist_id, stream_id, content_type],
    )?;
    Ok(())
}

#[tauri::command]
pub async fn remove_from_favorites(
    db: State<'_, DbConnection>,
    playlist_id: i64,
    stream_id: String,
) -> Result<(), Error> {
    let conn = db.0.lock().unwrap();
    conn.execute(
        "DELETE FROM favorites WHERE playlist_id = ?1 AND stream_id = ?2",
        params![playlist_id, stream_id],
    )?;
    Ok(())
}

#[tauri::command]
pub async fn get_favorites(db: State<'_, DbConnection>) -> Result<Vec<(i64, String)>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT playlist_id, stream_id FROM favorites")?;
    let favorites = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(favorites)
}
