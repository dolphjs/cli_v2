use std::error::Error;

use clap::Command;
use init::{init_command, init_dolph_cli};

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

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("dolph")
        .subcommand(init_command())
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let project_name = matches.value_of("PROJECT_NAME").unwrap();
        init_dolph_cli(project_name)?;
    }

    Ok(())
}
