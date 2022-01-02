use crate::cache;
use crate::Format;
use prettytable::{format, Table};

pub fn list(format: Format) {
    match format {
        Format::AsciiTable | Format::Text => {
            let mut table = Table::new();

            if format == Format::AsciiTable {
                table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                table.set_titles(row!["Project ID", "Name", "Favorite"]);
            } else {
                table.set_format(*format::consts::FORMAT_CLEAN);
            }

            for project in cache::read().projects {
                table.add_row(row![project.id, project.name, project.is_favorite]);
            }

            table.printstd();
        }
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&cache::read().projects).unwrap()
            );
        }
    }
}
