use clap::{arg, ArgMatches, Command};

use crate::{daemon::runner, utils::is_bin_installed};

use super::watcher;

pub fn init_watch_command(language: &str, matches: &ArgMatches) -> () {
    let use_bun = matches.is_present("bun");

    if use_bun {
        println!("Using Bun...");
        println!("Checking if system has bun installed...");
        if is_bin_installed("bun") {
            println!("Bun is installed. Using Bun...");
            watcher("development", "8080", language, true);
        } else {
            println!("Bun is not installed. Falling back to Node.js...");
            watcher("development", "8080", language, false);
        }
    } else {
        println!("Using Node...");
        println!("Checking if system has node.js installed...");

        if is_bin_installed("node") {
            println!("Node is installed. Using Node...");
            watcher("development", "8080", language, false);
        } else {
            println!("Node is not installed. Quitting...")
        }
    }
}

pub fn init_start_command(language: &str, matches: &ArgMatches) -> () {
    let use_bun = matches.is_present("bun");

    if use_bun {
        println!("Using Bun...");
        println!("Checking if system has bun installed...");
        if is_bin_installed("bun") {
            println!("Bun is installed. Using Bun...");
            runner("development", "8080", language, true);
        } else {
            println!("Bun is not installed. Falling back to Node.js...");
            runner("development", "8080", language, false);
        }
    } else {
        println!("Using Node...");
        println!("Checking if system has node.js installed...");

        if is_bin_installed("node") {
            println!("Node is installed. Using Node...");
            runner("development", "8080", language, false);
        } else {
            println!("Node is not installed. Quitting...")
        }
    }
}

pub fn watch_command() -> Command<'static> {
    Command::new("watch").about("Watch dolph.js server").arg(
        arg!(-b - -bun)
            .help("Uses bun as runtime to run the Dolph app")
            .required(false),
    )
}

pub fn start_command() -> Command<'static> {
    Command::new("start").about("Start dolph.js server").arg(
        arg!(-b - -bun)
            .help("Uses bun as runtime to run the Dolph app")
            .required(false),
    )
}

pub fn build_command() -> Command<'static> {
    Command::new("build").about("Build dolph.js ts project to js project for production")
}
