use prettytable::{format, Table};

use crate::cache;
use crate::Format;

pub fn list(format: Format) {
    match format {
        Format::AsciiTable | Format::Text => {
            let mut table = Table::new();

            if format == Format::AsciiTable {
                table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                table.set_titles(row!["Task ID", "Content"]);
            } else {
                table.set_format(*format::consts::FORMAT_CLEAN);
            }

            for task in cache::read().items {
                table.add_row(row![task.id, task.content]);
            }

            table.printstd();
        }
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&cache::read().items).unwrap()
            );
        }
    }
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
