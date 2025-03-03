use serde_json::Value;
use std::collections::HashMap;

pub fn extract_categories(api_data: &Value) -> HashMap<String, (String, String, Option<i64>)> {
    let mut all_categories = HashMap::new();

    // Try different JSON structures for categories
    println!("Attempting to extract categories from JSON data...");

    // Structure 1: panel_api.php format with nested categories
    if api_data["categories"].is_object() {
        println!("Found panel_api.php style categories structure");
        if let Some(live_categories) = api_data["categories"]["live"].as_array() {
            for cat in live_categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    let parent_id = cat["parent_id"].as_i64();
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "live".to_string(), parent_id),
                    );
                }
            }
        }
        if let Some(movie_categories) = api_data["categories"]["movie"].as_array() {
            for cat in movie_categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    let parent_id = cat["parent_id"].as_i64();
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "movie".to_string(), parent_id),
                    );
                }
            }
        }
        if let Some(series_categories) = api_data["categories"]["series"].as_array() {
            for cat in series_categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    let parent_id = cat["parent_id"].as_i64();
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "series".to_string(), parent_id),
                    );
                }
            }
        }
    }
    // Structure 2: player_api.php format with direct array
    else if api_data.is_array() {
        println!("Found player_api.php style array structure");
        if let Some(categories) = api_data.as_array() {
            for cat in categories {
                if let (Some(cat_id), Some(cat_name)) =
                    (cat["category_id"].as_str(), cat["category_name"].as_str())
                {
                    all_categories.insert(
                        cat_id.to_string(),
                        (cat_name.to_string(), "live".to_string(), None),
                    );
                }
            }
        }
    }

    println!("Extracted {} categories", all_categories.len());
    all_categories
}
