use reqwest::blocking::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub fn get_classes(api_key: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    println!("API key: {}", api_key);

    let url = "https://api.ucsb.edu/academics/curriculums/v1/classes/search?quarter=20202&pageNumber=1&pageSize=10&includeClassSections=true";

    let client = Client::new();
    let mut request_builder = client.get(url);
    request_builder = request_builder.header("accept", "application/json");
    request_builder = request_builder.header("ucsb-api-version", "1.0");
    request_builder = request_builder.header("ucsb-api-key", "g7a8HWVLDBiNZb3WJsp3ZcB7q5kD7aJI");

    let response = request_builder.send().unwrap();
    let response = response.json::<HashMap<String, Value>>().unwrap();
    Ok(response)
}
