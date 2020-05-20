extern crate flame;

use reqwest::blocking::Client;
use std::error::Error;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[path = "models/APIResponse.rs"]
mod api_response;

pub fn get_classes_threaded(api_key: String) -> Vec<api_response::course::Course> {
    let _guard = flame::start_guard("getting classes with multi-threading");
    let page_size = 100;
    let response = get_responses_paged(1, page_size, &api_key.clone());
    let remainder = if response.total % page_size > 0 { 1 } else { 0 };
    let total_page_count = (response.total / page_size) + remainder + 1;
    let mut all_classes = Vec::new();
    let max_threads = 8;
    let mut cur_page = 1;
    loop {
        let _guard = flame::start_guard(format!(
            "getting pages {}-{}",
            cur_page,
            cur_page + max_threads
        ));
        if cur_page > total_page_count {
            break;
        }
        let (tx, rx): (
            Sender<Vec<api_response::course::Course>>,
            Receiver<Vec<api_response::course::Course>>,
        ) = mpsc::channel();
        let mut child_threads = Vec::new();

        for page_num in 0..max_threads {
            let thread_tx = tx.clone();
            let api_key_clone = api_key.clone();
            println!("Spawning thread for page {}", page_num);
            let child_thread = thread::spawn(move || {
                let classes = get_classes_paged(page_num + cur_page, page_size, &api_key_clone);
                thread_tx.send(classes).unwrap();
            });

            child_threads.push(child_thread);
        }

        cur_page += max_threads;

        for child in child_threads {
            child.join().expect("child thread panicked!");
        }

        for i in 0..max_threads {
            println!("Waiting for response from thread {}", i);
            let page_of_classes = rx.recv().unwrap();
            println!("{}", page_of_classes.len());
            println!("Received response from thread {}", i);
            for class in page_of_classes {
                all_classes.push(class);
            }
        }
    }

    all_classes
}

pub fn get_classes_serially(
    api_key: &str,
) -> Result<Vec<api_response::course::Course>, Box<dyn Error>> {
    let mut classes = vec![];
    let mut page_number = 1;
    let page_size = 100;
    loop {
        let page_of_classes = get_classes_paged(page_number, page_size, api_key);
        if page_of_classes.is_empty() {
            break;
        } else {
            classes.extend(page_of_classes);
            page_number += 1;
        }
    }

    Ok(classes)
}

fn get_classes_paged(
    page_number: u32,
    page_size: u32,
    api_key: &str,
) -> Vec<api_response::course::Course> {
    get_responses_paged(page_number, page_size, api_key).classes
}

fn get_responses_paged(
    page_number: u32,
    page_size: u32,
    api_key: &str,
) -> api_response::APIResponse {
    let client = Client::new();
    let _guard = flame::start_guard(format!("getting page {}", page_number));

    let url = String::from(format!("https://api.ucsb.edu/academics/curriculums/v1/classes/search?quarter=20202&pageNumber={}&pageSize={}&includeClassSections=true", page_number, page_size));

    let mut request_builder = client.get(&url);
    request_builder = request_builder.header("accept", "application/json");
    request_builder = request_builder.header("ucsb-api-version", "1.0");
    request_builder = request_builder.header("ucsb-api-key", api_key);

    let response = {
        let _guard = flame::start_guard("making request to api");
        request_builder.send().unwrap()
    };
    let response = {
        let _guard = flame::start_guard("deserializing request json into objects");
        response.json::<api_response::APIResponse>().unwrap()
    };

    response
}
