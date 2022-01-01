use crate::cache;
use prettytable::{format, Table};
use serde::{Deserialize, Serialize};

pub fn list() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Project ID", "Name", "Favorite"]);

    for project in cache::read().projects {
        table.add_row(row![project.id, project.name, project.is_favorite]);
    }

    table.printstd();
}
