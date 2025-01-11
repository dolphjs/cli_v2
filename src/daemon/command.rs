use clap::{ArgMatches, Command};

use super::watcher;

pub fn init_watch_command(language: &str, matches: &ArgMatches) -> () {
    watcher("development", "8080", language);
}

pub fn watch_command() -> Command<'static> {
    Command::new("watch").about("Watch dolph.js server")
}
