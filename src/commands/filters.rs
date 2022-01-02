use crate::cache;
use crate::Format;
use prettytable::{format, Table};

pub fn list(format: Format) {
    match format {
        Format::AsciiTable | Format::Text => {
            let mut table = Table::new();

            if format == Format::AsciiTable {
                table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                table.set_titles(row!["ID", "Name", "Color", "Favorite", "Query"]);
            } else {
                table.set_format(*format::consts::FORMAT_CLEAN);
            }

            for filter in cache::read().filters {
                table.add_row(row![filter.id, filter.name, filter.color, filter.query]);
            }

            table.printstd();
        }
        Format::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&cache::read().filters).unwrap()
            );
        }
    }
}
