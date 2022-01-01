use prettytable::{format, Table};

use crate::cache;

pub fn list() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(row!["Task ID", "Content"]);

    for task in cache::read().items {
        table.add_row(row![task.id, task.content]);
    }

    table.printstd();
}

pub fn show(task_id: isize) {
    let cache = cache::read();
    for task in cache.items {
        if task.id == task_id {
            let mut table = Table::new();

            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

            table.add_row(row!["id", task.id]);
            table.add_row(row!["project_id", task.project_id]);
            table.add_row(row!["content", task.content]);
            table.add_row(row!["description", task.description]);
            table.add_row(row!["priority", task.priority]);

            table.printstd();

            return;
        }
    }
}
