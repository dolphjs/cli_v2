use clap::{arg, ArgMatches, Command};

use super::watcher;

pub fn init_watch_command(language: &str, matches: &ArgMatches) -> () {
    if matches.is_present("bun") {
        println!("Using Bun ...");
        println!("Checking if system has bun installed ...");
        watcher("development", "8080", language, true);
    } else {
        println!("Using Node ...");
        println!("Checking if system has node.js installed ...");
        watcher("development", "8080", language, false);
    }
}

pub fn watch_command() -> Command<'static> {
    Command::new("watch").about("Watch dolph.js server").arg(
        arg!(-b - -bun)
            .help("Uses bun as runtime to run the Dolph app")
            .required(false),
    )
}

pub fn build_command() -> Command<'static> {
    Command::new("build").about("Build dolph.js ts project to js project for production")
}
