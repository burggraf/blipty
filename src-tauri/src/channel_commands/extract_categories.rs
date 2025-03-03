use serde_json::Value;
use std::collections::HashSet;

pub fn extract_categories(data: &Value) -> Vec<(String, String)> {
    let mut categories: HashSet<(String, String)> = HashSet::new();

    if let Some(streams) = data.as_array() {
        for stream in streams {
            if let Some(category_id) = stream.get("category_id") {
                if let Some(category_id_str) = category_id.as_str() {
                    // Extract the category name from the stream object (if available)
                    let category_name = stream
                        .get("name")
                        .and_then(|name| name.as_str())
                        .unwrap_or("Unknown Category")
                        .to_string();

                    categories.insert((category_id_str.to_string(), category_name));
                }
            }
        }
    }

    // Convert the HashSet to a Vec
    let mut category_vec: Vec<(String, String)> = categories.into_iter().collect();

    // Sort the category vector by category ID
    category_vec.sort_by(|a, b| a.0.cmp(&b.0));

    category_vec
}
