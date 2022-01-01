use std::io::Write;

use std::env;
use std::fs::OpenOptions;
use std::path::Path;

pub fn sync() {
    let config_file = format!("{}/{}", env::var("HOME").unwrap(), ".todoist.cache");

    if Path::new(&config_file).is_file() {
        partial_sync();
    } else {
        full_sync();
    }
}

fn partial_sync() {
    println!("unimplemented!");
}

fn full_sync() {
    let cache_file = format!("{}/{}", env::var("HOME").unwrap(), ".todoist.cache");
    let token = format!("Bearer {}", String::from(env!("TODOIST_TOKEN")));
    let params = [("resource_types", "[\"all\"]")];

    let body = reqwest::blocking::Client::new()
        .post("https://api.todoist.com//sync/v8/sync")
        .header(reqwest::header::AUTHORIZATION, token)
        .form(&params)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(cache_file)
        .expect("to open the file");

    file.write_all(body.as_bytes()).expect("to write the file");
}
