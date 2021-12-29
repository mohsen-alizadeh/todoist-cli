use prettytable::{format, Table};
use serde::{Deserialize, Serialize};

use crate::client::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    id: isize,
    name: String,
    color: isize,
    order: isize,
    favorite: bool,
}

pub fn list() {
    let client = Client::new();

    let body = client.get("/rest/v1/labels");
    let labels: Vec<Label> = serde_json::from_str(&body).unwrap();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Label ID", "Name", "Color", "Order", "Favorite"]);

    for label in labels {
        table.add_row(row![
            label.id,
            label.name,
            label.color,
            label.order,
            label.favorite
        ]);
    }

    table.printstd();
}
