use rusqlite::params;
use std::collections::HashMap;
use tauri::State;

use crate::{db::DbConnection, models::Error};

pub fn insert_categories(
    db: State<'_, DbConnection>,
    all_categories: &HashMap<String, (String, String, Option<i64>)>,
) -> Result<(), Error> {
    // Insert categories into the database
    println!(
        "Inserting {} categories into the database",
        all_categories.len()
    );
    let mut conn = db.0.lock().unwrap();
    let tx = conn.transaction()?;
    for (cat_id, (cat_name, content_type, parent_id)) in all_categories {
        let result = tx.execute(
            "INSERT INTO categories (category_id, name, content_type, parent_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, strftime('%s', 'now'), strftime('%s', 'now'))",
            params![cat_id, cat_name, content_type, parent_id],
        );
        match result {
            Ok(_) => {
                println!("Inserted category: {}", cat_name);
            }
            Err(e) => {
                println!("Error inserting category {}: {}", cat_name, e);
            }
        }
    }
    tx.commit()?;
    Ok(())
}
