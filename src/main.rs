#[macro_use]
extern crate prettytable;

use clap::{App, AppSettings, Arg, SubCommand};
use env_logger;
use log::{debug, info};
// use log::{debug, info};

mod client;
mod commands;

use commands::{labels, projects, sync, tasks};

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
        .get_matches();

    match app.subcommand() {
        ("list", Some(_)) => tasks::list(),
        ("projects", Some(_)) => projects::list(),
        ("labels", Some(_)) => labels::list(),
        ("sync", Some(_)) => sync(),
        ("show", Some(args)) => {
            let id: isize = args.value_of("task_id").unwrap().parse().unwrap();
            tasks::show(id)
        }
        _ => (),
    }
}
