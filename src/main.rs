use std::error::Error;

use clap::Command;
use daemon::{build_command, build_ts_app, init_watch_command, watch_command};
use init::{init_command, init_dolph_cli};
use properties::{init_architecture, run_init_architecture};
use utils::read_config;

mod daemon;
mod init;
mod properties;
mod utils;
mod writers;

#[derive(Debug)]
pub enum ServerFileError {
    BaseDirectoryNotFound,
    InvalidDatabase(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for ServerFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerFileError::BaseDirectoryNotFound => write!(f, "Base directory not found"),
            ServerFileError::InvalidDatabase(db) => write!(f, "Invalid database type: {}", db),
            ServerFileError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Error for ServerFileError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("dolph")
        .subcommand(init_command())
        .subcommand(init_architecture())
        .subcommand(watch_command())
        .subcommand(build_command())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches.value_of("PROJECT_NAME").unwrap();
        init_dolph_cli(project_name)?;
    } else if let Some(matchess) = matches.subcommand_matches("generate") {
        match read_config() {
            Ok(config) => {
                let generator = properties::Generator::new(config);
                run_init_architecture(generator, matchess).await?;
            }
            Err(e) => eprintln!("Failed to read config file: {}", e),
        }
    } else if let Some(matchess) = matches.subcommand_matches("watch") {
        match read_config() {
            Ok(config) => {
                init_watch_command(&config.language, matchess);
            }
            Err(e) => eprintln!("Failed to read config file: {}", e),
        }
    } else if let Some(matchess) = matches.subcommand_matches("build") {
        match read_config() {
            Ok(config) => {
                let _ = build_ts_app(&config.language).await;
            }
            Err(e) => eprintln!("Failed to read config file: {}", e),
        }
    }

    Ok(())
}
