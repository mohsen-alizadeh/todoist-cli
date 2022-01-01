use prettytable::{format, Table};

use crate::cache;

pub fn list() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["ID", "Name", "Color", "Favorite", "Query"]);

    for filter in cache::read().filters {
        table.add_row(row![filter.id, filter.name, filter.color, filter.query]);
    }

    table.printstd();
}
