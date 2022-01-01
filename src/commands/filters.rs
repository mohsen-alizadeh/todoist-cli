// use std::fs;
use prettytable::{format, Table};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    id: isize,
    color: isize,
    is_deleted: isize,
    is_favorite: isize,
    item_order: isize,
    name: String,
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    filters: Vec<Filter>,
}

pub fn list() {
    let config_file = format!("{}/{}", env::var("HOME").unwrap(), ".todoist.cache");

    let cache = fs::read_to_string(config_file).unwrap();

    let cache: Cache = serde_json::from_str(&cache).unwrap();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["ID", "Name", "Color", "Favorite", "Query"]);

    for filter in cache.filters {
        table.add_row(row![filter.id, filter.name, filter.color, filter.query]);
    }

    table.printstd();
}
