use std::io::Write;

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub id: isize,
    pub color: isize,
    pub is_deleted: isize,
    pub is_favorite: isize,
    pub item_order: isize,
    pub name: String,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    pub filters: Vec<Filter>,
}

pub fn read() -> Cache {
    let config_file = format!("{}/{}", env::var("HOME").unwrap(), ".todoist.cache");

    let cache = fs::read_to_string(config_file).unwrap();

    let cache: Cache = serde_json::from_str(&cache).unwrap();

    cache
}

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
