extern crate flame;

use std::collections::HashMap;
use std::fs::File;

mod ucsb_api_service;

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("secrets")).unwrap();
    let settings = settings.try_into::<HashMap<String, String>>().unwrap();
    let api_key = settings.get("api_key").unwrap();

    let classes = {
        let _guard = flame::start_guard("retrieving ALL classes");
        ucsb_api_service::get_classes(api_key)
    };

    let json = {
        let _guard = flame::start_guard("serializing list of classes to json");
        serde_json::to_string(&classes).unwrap()
    };

    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    println!("{}", json);
}
