use prettytable::{format, Table};
use serde::{Deserialize, Serialize};

use crate::client::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    id: isize,
    order: Option<isize>,
    color: isize,
    name: String,
    comment_count: isize,
    shared: bool,
    favorite: bool,
    sync_id: isize,
    url: String,
    inbox_project: Option<bool>,
}

pub fn list() {
    let client = Client::new();

    let body = client.get("/rest/v1/projects");
    let projects: Vec<Project> = serde_json::from_str(&body).unwrap();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Project ID", "Name", "Favorite"]);

    for project in projects {
        table.add_row(row![project.id, project.name, project.favorite]);
    }

    table.printstd();
}
