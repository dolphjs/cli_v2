use clap::{arg, Command};
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

use crate::properties::DefaultConfig;
use crate::writers::{
    write_datasource_config, write_dolph_config, write_gitignore, write_graphql_server_file,
    write_package_json, write_setup_file, write_spring_server_file, write_swcrc, write_tsconfig,
};

pub fn init_command() -> Command<'static> {
    Command::new("new")
        .alias("nw")
        .about("nw")
        .about("Creates a new dolphjs app")
        .arg(arg!(<PROJECT_NAME> "The name of the project to create"))
}

pub fn init_dolph_cli(app_name: &str) -> Result<(), Box<dyn Error>> {
    if app_name.is_empty() {
        println!("Error: provide a name for your dolph project or indicate with a '.' to use current directory.");
        process::exit(1);
    }

    let project_name = if app_name == "." {
        Path::new(".")
            .canonicalize()?
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned()
    } else {
        fs::create_dir(app_name)?;
        std::env::set_current_dir(app_name)?;
        app_name.to_string()
    };

    let current_dir = std::env::current_dir()?;
    let src_path = current_dir.join("src");
    let config_path = current_dir.join("dolph_cli.yaml");
    let test_path = current_dir.join("tests");
    let shared_path = src_path.join("shared");
    let component_path = src_path.join("components");

    // Create necessary directories
    fs::create_dir_all(&src_path)?;
    fs::create_dir_all(&test_path)?;

    if !config_path.exists() {
        let theme = ColorfulTheme::default();

        let api_options = ["graphql", "rest"];
        let api = api_options[Select::with_theme(&theme)
            .with_prompt("Will you be using REST or GraphQL to build?")
            .default(1)
            .items(&api_options)
            .interact()?];

        let language: &str;
        let routing: &str;

        if api.to_string() == "rest" {
            let routing_options = ["express", "spring"];
            routing = routing_options[Select::with_theme(&theme)
                .with_prompt("What dolph routing are you using?")
                .default(0)
                .items(&routing_options)
                .interact()?];

            let language_options = ["ts", "js"];
            language = language_options[Select::with_theme(&theme)
                .with_prompt("Select your preferred language")
                .default(0)
                .items(&language_options)
                .interact()?];
        } else {
            routing = "spring";
            language = "ts";
        }

        let database_options = ["mongo", "mysql", "postgresql", "other"];
        let database = database_options[Select::with_theme(&theme)
            .with_prompt("What is your database of choice?")
            .default(0)
            .items(&database_options)
            .interact()?];

        let mut config = DefaultConfig {
            language: language.to_string(),
            api: api.to_string(),
            database: database.to_string(),
            routing: routing.to_string(),
        };

        let yaml_content = format!(
            "# this is an auto-generated file, please do not edit manually\n{}",
            serde_yaml::to_string(&config)?
        );
        fs::write(&config_path, yaml_content)?;

        // Create additional directories for spring routing
        if config.routing == "spring" {
            fs::create_dir_all(&component_path)?;
            fs::create_dir_all(&shared_path)?;

            if config.api == "graphql" {
                write_datasource_config(&mut config.database)?;
                write_setup_file()?;
                write_graphql_server_file()?;
            } else {
                write_spring_server_file(&config.database, &String::from(""))?;
            }
        }

        if config.language == "ts" {
            write_swcrc(config.routing == "spring")?;
            write_tsconfig(config.routing == "spring")?;
        }

        write_dolph_config()?;
        write_package_json(&project_name, &config.language)?;
        write_gitignore()?;

        println!("dolph configurations have been initialized successfully. ✨");
        println!("run `yarn dev:start` to build the project and start development. 🚀");
    } else {
        println!("I see you already have your dolph configurations present, so I'll abort gracefully... 😉");
    }

    Ok(())
}
