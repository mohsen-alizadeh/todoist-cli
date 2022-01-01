use crate::cache;
use prettytable::{format, Table};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    id: isize,
    name: String,
    color: isize,
    order: isize,
    favorite: bool,
}

pub fn list() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Label ID", "Name", "Color", "Order", "Favorite"]);

    for label in cache::read().labels {
        table.add_row(row![
            label.id,
            label.name,
            label.color,
            label.item_order,
            label.is_favorite
        ]);
    }

    table.printstd();
}
