extern crate flame;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod ucsb_api_service;

fn main() {
    let guard = flame::start_guard("main");
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("secrets")).unwrap();
    let settings = settings.try_into::<HashMap<String, String>>().unwrap();
    let api_key = settings.get("api_key").unwrap();

    let classes_serial = {
        let _guard = flame::start_guard("retrieving all classes serially");
        ucsb_api_service::get_classes_serially(api_key).unwrap()
    };

    let _classes_threaded = {
        let _guard = flame::start_guard("retrieving all classes with threads");
        ucsb_api_service::get_classes_threaded(String::from(api_key))
    };

    let json = {
        let _guard = flame::start_guard("serializing list of classes to json");
        serde_json::to_string(&classes_serial).unwrap()
    };

    let file_name = "classes.json";
    serialize_classes(&json, file_name).unwrap();
    guard.end();
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}

fn serialize_classes(json: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let _guard = flame::start_guard("serialize_classes");
    let mut file;
    {
        let _guard = flame::start_guard("creating classes.json");
        file = match File::create(&Path::new(file_name)) {
            Err(e) => panic!("{}", e),
            Ok(file) => file,
        };
    }
    {
        let _guard = flame::start_guard("writing to classes.json");
        match file.write_all(&json.as_bytes()) {
            Err(e) => panic!("{}", e),
            Ok(_) => println!("Successfully serialized classes to a file"),
        };
    }
    Ok(())
}
