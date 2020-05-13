use reqwest::blocking::Client;
use std::error::Error;

#[path = "models/APIResponse.rs"]
mod api_response;

pub fn get_classes(api_key: &str) -> Vec<api_response::course::Course> {
    let response = match get_classes_response(api_key) {
        Err(_e) => Vec::new(),
        Ok(response) => response,
    };
    response
}

fn get_classes_response(
    api_key: &str,
) -> Result<Vec<api_response::course::Course>, Box<dyn Error>> {
    let client = Client::new();
    let mut classes = vec![];
    let mut page_number = 1;
    let page_size = 100;
    loop {
        println!("getting page {}", page_number);
        let url = String::from(format!("https://api.ucsb.edu/academics/curriculums/v1/classes/search?quarter=20202&pageNumber={}&pageSize={}&includeClassSections=true", page_number, page_size));
        let mut request_builder = client.get(&url);
        request_builder = request_builder.header("accept", "application/json");
        request_builder = request_builder.header("ucsb-api-version", "1.0");
        request_builder = request_builder.header("ucsb-api-key", api_key);

        let response = request_builder.send().unwrap();
        let response = response.json::<api_response::APIResponse>().unwrap();
        if response.classes.is_empty() {
            break;
        } else {
            classes.extend(response.classes.iter().cloned());
            // classes.append(&mut response.classes);
            page_number += 1;
        }
    }

    Ok(classes)
}
