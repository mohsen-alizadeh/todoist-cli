#[macro_use]
extern crate prettytable;

use clap::{App, AppSettings, SubCommand};

mod client;
mod commands;

use commands::{projects, tasks};

fn main() {
    let app = App::new("Todoist Cli")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Todoist Cli")
        .subcommand(SubCommand::with_name("list").about("List tasks"))
        .subcommand(SubCommand::with_name("show").about("Show a task"))
        .subcommand(SubCommand::with_name("projects").about("List of projects"))
        .get_matches();

    match app.subcommand() {
        ("list", Some(_)) => tasks::list(),
        ("projects", Some(_)) => projects::list(),
        _ => (),
    }
}
