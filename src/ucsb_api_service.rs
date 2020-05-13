use reqwest::blocking::Client;
use reqwest::blocking::Response;
use std::error::Error;

#[path = "models/APIResponse.rs"]
mod api_response;

pub fn get_classes(api_key: &str) -> Result<api_response::APIResponse, Box<dyn Error>> {
    let response = get_classes_response(api_key);
    let response = response
        .unwrap()
        .json::<api_response::APIResponse>()
        .unwrap();
    Ok(response)
}

fn get_classes_response(api_key: &str) -> Result<Response, Box<dyn Error>> {
    let url = "https://api.ucsb.edu/academics/curriculums/v1/classes/search?quarter=20202&pageNumber=1&pageSize=10&includeClassSections=true";

    let client = Client::new();
    let mut request_builder = client.get(url);
    request_builder = request_builder.header("accept", "application/json");
    request_builder = request_builder.header("ucsb-api-version", "1.0");
    request_builder = request_builder.header("ucsb-api-key", api_key);

    let response = request_builder.send().unwrap();

    Ok(response)
}
