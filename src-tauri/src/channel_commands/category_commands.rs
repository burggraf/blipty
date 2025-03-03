use tauri::State;

use crate::{db::DbConnection, models::Error};

#[tauri::command(rename_all = "camelCase")]
pub async fn get_categories(
    db: State<'_, DbConnection>,
    playlist_id: i64,
) -> Result<Vec<String>, Error> {
    let conn = db.0.lock().unwrap();
    let mut stmt = conn.prepare("SELECT DISTINCT category FROM channels WHERE playlist_id = ?")?;
    let categories = stmt
        .query_map([playlist_id], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(categories)
}
