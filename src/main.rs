use clap::{App, Arg, SubCommand};
use std::collections::HashMap;

mod client;
mod commands;

fn main() {
    let app = App::new("Todoist Cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Todoist Cli")
        .subcommand(SubCommand::with_name("list").about("List tasks"))
        .subcommand(SubCommand::with_name("show").about("Show a task"))
        .subcommand(SubCommand::with_name("projects").about("List of projects"))
        .arg(Arg::with_name("file_name"))
        .get_matches();

    match app.subcommand() {
        ("list", Some(list_sub)) => commands::list(),
        _ => (),
    }
}
