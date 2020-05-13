use std::collections::HashMap;

mod ucsb_api_service;

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("secrets")).unwrap();
    let settings = settings.try_into::<HashMap<String, String>>().unwrap();
    let api_key = settings.get("api_key").unwrap();

    let classes = ucsb_api_service::get_classes(api_key).unwrap();

    println!("{:#?}", classes);
}
