use std::error::Error;

use clap::{arg, Command};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Generator {
    config: DefaultConfig,
}

#[derive(Debug, Serialize, Deserialize)]
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
        println!("Generating controller: {}", name);
        Ok(())
    }

    pub async fn generate_service(&self, name: &str) -> Result<()> {
        println!("Generating service: {}", name);
        Ok(())
    }

    pub async fn generate_model(&self, name: &str) -> Result<()> {
        println!("Generating model: {}", name);
        Ok(())
    }

    pub async fn generate_route(&self, name: &str) -> Result<()> {
        println!("Generating route: {}", name);
        Ok(())
    }

    pub async fn generate_component(&self, name: &str) -> Result<()> {
        println!("Generating component: {}", name);
        Ok(())
    }

    pub async fn generate_socket(&self, name: &str) -> Result<()> {
        println!("Generating socket: {}", name);
        Ok(())
    }

    pub async fn generate_resolver(&self, name: &str) -> Result<()> {
        println!("Generating resolver: {}", name);
        Ok(())
    }

    pub async fn generate_entity(&self, name: &str) -> Result<()> {
        println!("Generating entity: {}", name);
        Ok(())
    }

    pub async fn generate_dto(&self, name: &str) -> Result<()> {
        println!("Generating dto: {}", name);
        Ok(())
    }

    pub async fn generate_server(&self, name: &str) -> Result<()> {
        println!("Generating server: {}", name);
        Ok(())
    }

    pub async fn generate_all(&self, name: &str) -> Result<()> {
        self.generate_dto(name).await?;
        self.generate_service(name).await?;

        if self.config.api.to_string() == String::from("rest") {
            self.generate_controller(name).await?;
            self.generate_model(name).await?;
        } else if self.config.api.to_string() == String::from("graphql") {
            self.generate_entity(name).await?;
            self.generate_resolver(name).await?;
        }

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

        Ok(())
    }
}

pub fn init_architecture() -> Command<'static> {
    Command::new("dolph").about("DolphJS CLI").subcommand(
        Command::new("generate")
            .arg(
                arg!(--service -s <NAME>)
                    .help("Generates a dolphjs service file")
                    .required(false),
            )
            .arg(
                arg!(--controller -c <NAME>)
                    .help("Generates a dolphjs controller file")
                    .required(false),
            )
            .arg(
                arg!(--route -r <NAME>)
                    .help("Generates a dolphjs routes file")
                    .required(false),
            )
            .arg(
                arg!(--model -m <NAME>)
                    .help("Generates a dolphjs models file")
                    .required(false),
            )
            .arg(
                arg!(--entity -e <NAME>)
                    .help("Generates a dolphjs entity file")
                    .required(false),
            )
            .arg(
                arg!(--dto -d <NAME>)
                    .help("Generates a dolphjs dto file")
                    .required(false),
            )
            .arg(
                arg!(--component -com <NAME>)
                    .help("Generates a dolphjs spring component file")
                    .required(false),
            )
            .arg(
                arg!(--socket -soc <NAME>)
                    .help("Generate a dolphjs socket service and component")
                    .required(false),
            )
            .arg(
                arg!(--all -a <NAME>)
                    .help("Generates all dolphjs files for the named parameter")
                    .required(false),
            ),
    )
}

pub async fn run_init_architecture(generator: Generator) -> Result<()> {
    let matches = init_architecture().get_matches();

    if let Some(generate_matches) = matches.subcommand_matches("generate") {
        if let Some(name) = generate_matches.value_of("controller") {
            generator.generate_controller(name).await?;
        }

        if let Some(name) = generate_matches.value_of("service") {
            generator.generate_service(name).await?;
        }

        if let Some(name) = generate_matches.value_of("route") {
            if generator.config.routing == "spring" {
                println!("Cannot create routes file for spring routing");
            } else {
                generator.generate_route(name).await?;
            }
        }

        if let Some(name) = generate_matches.value_of("model") {
            generator.generate_model(name).await?;
        }

        if let Some(name) = generate_matches.value_of("entity") {
            generator.generate_entity(name).await?;
        }

        if let Some(name) = generate_matches.value_of("dto") {
            generator.generate_dto(name).await?;
        }

        if let Some(name) = generate_matches.value_of("resolver") {
            generator.generate_resolver(name).await?;
        }

        if let Some(name) = generate_matches.value_of("socket") {
            generator.generate_socket(name).await?;
        }

        if let Some(name) = generate_matches.value_of("component") {
            generator.generate_component(name).await?;
        }

        if let Some(name) = generate_matches.value_of("all") {
            generator.generate_all(name).await?;
        }
    }

    Ok(())
}
