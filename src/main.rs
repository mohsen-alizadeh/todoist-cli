#[macro_use]
extern crate prettytable;

use clap::{App, AppSettings, Arg, SubCommand};
use env_logger;
use log::debug;

mod cache;
mod commands;

use commands::{filters, labels, projects, tasks};

#[derive(Debug, PartialEq)]
pub enum Format {
    Json,
    AsciiTable,
    Text,
}

fn main() {
    env_logger::init();

    debug!("main is started");

    let app = App::new("Todoist Cli")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Todoist Cli")
        .subcommand(SubCommand::with_name("list").about("List tasks"))
        .subcommand(
            SubCommand::with_name("show")
                .about("Show a task")
                .arg(Arg::with_name("task_id")),
        )
        .subcommand(SubCommand::with_name("projects").about("List of projects"))
        .subcommand(SubCommand::with_name("labels").about("List of labels"))
        .subcommand(SubCommand::with_name("sync").about("sync with Todoist"))
        .subcommand(SubCommand::with_name("filters").about("List of filters"))
        .arg(Arg::with_name("json").short("j").long("json"))
        .arg(Arg::with_name("text").short("t").long("text"))
        .arg(
            Arg::with_name("assci-table")
                .short("a")
                .long("assci-table")
                .help("Prints assci table"),
        )
        .get_matches();

    let format = if app.is_present("json") {
        Format::Json
    } else if app.is_present("text") {
        Format::Text
    } else {
        Format::AsciiTable
    };

    match app.subcommand() {
        ("list", Some(_)) => tasks::list(format),
        ("filters", Some(_)) => filters::list(format),
        ("projects", Some(_)) => projects::list(format),
        ("labels", Some(_)) => labels::list(format),
        ("show", Some(args)) => {
            let id: isize = args.value_of("task_id").unwrap().parse().unwrap();
            tasks::show(id)
        }
        _ => (),
    }
}
