use reqwest;
use serde_json::Value;
use std::collections::HashMap;

use crate::channel_commands::extract_categories::extract_categories;
use crate::models::Error;

pub async fn fetch_api_data(
    server_url: String,
    username: String,
    password: String,
) -> Result<(Value, HashMap<String, String>), Error> {
    let mut api_data = Value::Null;
    let mut categories: HashMap<String, String> = HashMap::new();
    let client = reqwest::Client::new();

    // Fetch categories
    let categories_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_live_categories",
        server_url, username, password
    );

    println!("Trying categories endpoint: {}", categories_endpoint);
    match client.get(&categories_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to categories endpoint: {}",
                    categories_endpoint
                );
                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed categories JSON data");
                        if let Some(categories_array) = data.as_array() {
                            for category in categories_array {
                                if let Some(category_id) =
                                    category.get("category_id").and_then(|v| v.as_str())
                                {
                                    if let Some(category_name) =
                                        category.get("category_name").and_then(|v| v.as_str())
                                    {
                                        categories.insert(
                                            category_id.to_string(),
                                            category_name.to_string(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!(
                            "Failed to parse categories JSON from {}: {}",
                            categories_endpoint, e
                        );
                    }
                }
            } else {
                println!(
                    "Failed to connect to categories endpoint {}: {}",
                    categories_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to categories endpoint {}: {}",
                categories_endpoint, e
            );
        }
    }

    // Fetch streams
    let streams_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_live_streams",
        server_url, username, password
    );

    println!("Trying streams endpoint: {}", streams_endpoint);
    match client.get(&streams_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to streams endpoint: {}",
                    streams_endpoint
                );

                // For JSON API formats
                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed streams JSON data");
                        // Print the top-level structure of the JSON
                        if let Some(obj) = data.as_object() {
                            println!("JSON structure has the following top-level keys:");
                            for (key, value) in obj {
                                let type_str = match value {
                                    Value::Null => "null",
                                    Value::Bool(_) => "boolean",
                                    Value::Number(_) => "number",
                                    Value::String(_) => "string",
                                    Value::Array(_) => "array",
                                    Value::Object(_) => "object",
                                };
                                println!("  - {}: {}", key, type_str);
                            }
                        } else {
                            println!("JSON data is not an object, it's a: {:?}", data);
                        }
                        api_data = data;
                    }
                    Err(e) => {
                        println!("Failed to parse JSON from {}: {}", streams_endpoint, e);
                    }
                }
            } else {
                println!(
                    "Failed to connect to streams endpoint {}: {}",
                    streams_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to streams endpoint {}: {}",
                streams_endpoint, e
            );
        }
    }

    Ok((api_data, categories))
}
