use prettytable::{format, Cell, Row, Table};

use crate::client::Client;

pub fn list() {
    let client = Client::new();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Task ID", "Content"]);

    for task in client.list() {
        table.add_row(row![task.id, task.content]);
    }

    table.printstd();
}
