use std::error::Error;

use clap::{arg, ArgMatches, Command};

use serde::{Deserialize, Serialize};

use crate::writers::{
    write_db_config, write_entity, write_graphql_service, write_input, write_resolver, write_socket_service, write_spring_component, write_spring_controller, write_spring_dto, write_spring_model, write_spring_server_file, write_spring_service
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Generator {
    config: DefaultConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultConfig {
    pub language: String,
    pub database: String,
    pub routing: String,
    pub api: String,
}

impl DefaultConfig {
    pub fn new(config: DefaultConfig) -> Self {
        Self {
            language: config.language.to_string(),
            database: config.database.to_string(),
            routing: config.routing.to_string(),
            api: config.api.to_string(),
        }
    }
}

impl Generator {
    pub fn new(config: DefaultConfig) -> Self {
        Self { config }
    }

    pub async fn generate_controller(&self, name: &str) -> Result<()> {
        write_spring_controller(name)?;
        println!("Generated controller: {}", name);
        Ok(())
    }

    pub async fn generate_service(&self, name: &str) -> Result<()> {
        if self.config.api == "graphql" {
            write_graphql_service(name)?;
        } else {
            write_spring_service(&self.config.database, name)?;
        }

        println!("Generated service: {}", name);
        Ok(())
    }

    pub async fn generate_model(&self, name: &str) -> Result<()> {
        if self.config.database == "mysql" {
            self.generate_db_config(name).await?;
        }
        write_spring_model(&self.config.database, name)?;
        println!("Generated model: {}", name);
        Ok(())
    }

    pub async fn generate_route(&self, name: &str) -> Result<()> {
        println!("Generated route: {}", name);
        Ok(())
    }

    pub async fn generate_component(&self, name: &str) -> Result<()> {
        write_spring_component(name)?;
        println!("Generated component: {} component", name);
        Ok(())
    }

    pub async fn generate_socket(&self, name: &str) -> Result<()> {
        write_socket_service(name)?;
        println!("Generated socket: {}", name);
        Ok(())
    }

    pub async fn generate_resolver(&self, name: &str) -> Result<()> {
        write_resolver(name)?;
        println!("Generated resolver: {}", name);
        Ok(())
    }

    pub async fn generate_input(&self, name: &str) -> Result<()> {
        write_input(name)?;
        println!("Generated input: {}", name);
        Ok(())
    }

    pub async fn generate_entity(&self, name: &str) -> Result<()> {
        write_entity(name)?;
        println!("Generated entity: {}", name);
        Ok(())
    }

    pub async fn generate_dto(&self, name: &str) -> Result<()> {
        write_spring_dto(name)?;
        println!("Generated dto: {}", name);
        Ok(())
    }

    pub async fn generate_db_config(&self, name: &str) -> Result<()> {
        write_db_config(name)?;
        println!("Generated db config: {}", name);
        Ok(())
    }

    pub async fn generate_server(&self, name: &str) -> Result<()> {
        write_spring_server_file(&self.config.database, name)?;
        println!("Generated server: {}", name);
        Ok(())
    }

    pub async fn generate_all(&self, name: &str) -> Result<()> {
        self.generate_service(name).await?;

        if self.config.api.to_string() == String::from("rest") {
            self.generate_dto(name).await?;
            self.generate_controller(name).await?;
            self.generate_model(name).await?;

            match self.config.routing.as_str() {
                "express" => {
                    self.generate_route(name).await?;
                    // Add route to index file
                    // Add server file
                }
                "spring" => {
                    self.generate_component(name).await?;
                    // Add controller to component file
                    // Add component to server file
                    // Add server file
                }
                _ => println!("Unknown routing type"),
            }
        } else if self.config.api.to_string() == String::from("graphql") {
            self.generate_entity(name).await?;
            self.generate_resolver(name).await?;
            self.generate_input(name).await?;
        }

        Ok(())
    }
}

pub fn init_architecture() -> Command<'static> {
    Command::new("generate")
        .about("DolphJS CLI")
        .arg(
            arg!(-s --service <NAME>)
                .help("Generates a dolphjs service file")
                .required(false),
        )
        .arg(
            arg!(-c --controller <NAME>)
                .help("Generates a dolphjs controller file")
                .required(false),
        )
        .arg(
            arg!(-r --route <NAME>)
                .help("Generates a dolphjs routes file")
                .required(false),
        )
        .arg(
            arg!(-m --model <NAME>)
                .help("Generates a dolphjs models file")
                .required(false),
        )
        .arg(
            arg!(-e --entity <NAME>)
                .help("Generates a dolphjs entity file")
                .required(false),
        )
        .arg(
            arg!(-i --input <NAME>)
                .help("Generates a dolphjs input file")
                .required(false),
        )
        .arg(
            arg!(-d --dto <NAME>)
                .help("Generates a dolphjs dto file")
                .required(false),
        )
        .arg(
            arg!(-v --resolver <NAME>)
                .help("Generates a dolphjs resolver file")
                .required(false),
        )
        .arg(
            arg!(-y --component <NAME>)
                .help("Generates a dolphjs spring component file")
                .required(false),
        )
        .arg(
            arg!(-k --socket <NAME>)
                .help("Generate a dolphjs socket service and component")
                .required(false),
        )
        .arg(
            arg!(-a --all <NAME>)
                .help("Generates all dolphjs files for the named parameter")
                .required(false),
        )
}

pub async fn run_init_architecture(generator: Generator, matches: &ArgMatches) -> Result<()> {
    if let Some(name) = matches.value_of("controller") {
        generator.generate_controller(name).await?;
    }

    if let Some(name) = matches.value_of("service") {
        generator.generate_service(name).await?;
    }

    if let Some(name) = matches.value_of("route") {
        if generator.config.routing == "spring" {
            println!("Cannot create routes file for spring routing");
        } else {
            generator.generate_route(name).await?;
        }
    }

    if let Some(name) = matches.value_of("model") {
        generator.generate_model(name).await?;
    }

    if let Some(name) = matches.value_of("entity") {
        generator.generate_entity(name).await?;
    }

    if let Some(name) = matches.value_of("dto") {
        generator.generate_dto(name).await?;
    }

    if let Some(name) = matches.value_of("input") {
        generator.generate_input(name).await?;
    }

    if let Some(name) = matches.value_of("resolver") {
        generator.generate_resolver(name).await?;
    }

    if let Some(name) = matches.value_of("socket") {
        generator.generate_socket(name).await?;
    }

    if let Some(name) = matches.value_of("component") {
        generator.generate_component(name).await?;
    }

    if let Some(name) = matches.value_of("all") {
        generator.generate_all(name).await?;
    }

    Ok(())
}
