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
    let client = reqwest::Client::new();
    let mut live_categories: HashMap<String, String> = HashMap::new();
    let mut vod_categories: HashMap<String, String> = HashMap::new();

    // Fetch categories
    let live_categories_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_live_categories",
        server_url, username, password
    );

    println!(
        "Trying live categories endpoint: {}",
        live_categories_endpoint
    );

    match client.get(&live_categories_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to live categories endpoint: {}",
                    live_categories_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed live categories JSON data");

                        if let Some(categories_array) = data.as_array() {
                            for category in categories_array {
                                if let Some(category_id) =
                                    category.get("category_id").and_then(|v| v.as_str())
                                {
                                    if let Some(category_name) =
                                        category.get("category_name").and_then(|v| v.as_str())
                                    {
                                        live_categories.insert(
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
                            "Failed to parse live categories JSON from {}: {}",
                            live_categories_endpoint, e
                        );
                    }
                }
            } else {
                println!(
                    "Failed to connect to live categories endpoint {}: {}",
                    live_categories_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to live categories endpoint {}: {}",
                live_categories_endpoint, e
            );
        }
    }

    // Fetch VOD categories
    let vod_categories_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_vod_categories",
        server_url, username, password
    );

    println!(
        "Trying VOD categories endpoint: {}",
        vod_categories_endpoint
    );

    match client.get(&vod_categories_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to VOD categories endpoint: {}",
                    vod_categories_endpoint
                );

                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed VOD categories JSON data");

                        if let Some(categories_array) = data.as_array() {
                            for category in categories_array {
                                if let Some(category_id) =
                                    category.get("category_id").and_then(|v| v.as_str())
                                {
                                    if let Some(category_name) =
                                        category.get("category_name").and_then(|v| v.as_str())
                                    {
                                        vod_categories.insert(
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
                            "Failed to parse VOD categories JSON from {}: {}",
                            vod_categories_endpoint, e
                        );
                    }
                }
            } else {
                println!(
                    "Failed to connect to VOD categories endpoint {}: {}",
                    vod_categories_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to VOD categories endpoint {}: {}",
                vod_categories_endpoint, e
            );
        }
    }

    // Fetch live streams
    let live_streams_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_live_streams",
        server_url, username, password
    );

    println!("Trying live streams endpoint: {}", live_streams_endpoint);

    match client.get(&live_streams_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to live streams endpoint: {}",
                    live_streams_endpoint
                );

                // For JSON API formats
                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed live streams JSON data");

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
                        println!("Failed to parse JSON from {}: {}", live_streams_endpoint, e);
                    }
                }
            } else {
                println!(
                    "Failed to connect to live streams endpoint {}: {}",
                    live_streams_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to live streams endpoint {}: {}",
                live_streams_endpoint, e
            );
        }
    }

    // Fetch vod streams
    let vod_streams_endpoint = format!(
        "{}/player_api.php?username={}&password={}&action=get_vod_streams",
        server_url, username, password
    );

    println!("Trying vod streams endpoint: {}", vod_streams_endpoint);

    match client.get(&vod_streams_endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                println!(
                    "Successfully connected to vod streams endpoint: {}",
                    vod_streams_endpoint
                );

                // For JSON API formats
                match response.json::<Value>().await {
                    Ok(data) => {
                        println!("Successfully parsed vod streams JSON data");

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
                        println!("Failed to parse JSON from {}: {}", vod_streams_endpoint, e);
                    }
                }
            } else {
                println!(
                    "Failed to connect to vod streams endpoint {}: {}",
                    vod_streams_endpoint,
                    response.status()
                );
            }
        }
        Err(e) => {
            println!(
                "Error connecting to vod streams endpoint {}: {}",
                vod_streams_endpoint, e
            );
        }
    }

    // Combine live and VOD categories
    let mut all_categories: HashMap<String, String> = HashMap::new();
    all_categories.extend(live_categories);
    all_categories.extend(vod_categories);

    Ok((api_data, all_categories))
}
