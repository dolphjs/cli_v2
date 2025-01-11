use std::{error::Error, fs};

use crate::utils::ensure_directory_exists;

use super::config_writers::{find_base_directory, Database};

pub fn write_datasource_config(mut database: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let shared_path = base_directory.join("shared");

    ensure_directory_exists(&shared_path)?;

    let configs_path = shared_path.join("configs");

    ensure_directory_exists(&configs_path)?;

    let index_path = configs_path.join("data_source.ts");

    let import_statement = r#"import { DataSource } from "typeorm";
    "#
    .to_string();

    let mut db = database;

    match &mut database {
        &mut "mysql" => db = "mysql",
        &mut "mongo" => db = "mongo",
        &mut "postgresql" => db = "postgres",
        &mut "other" => db = "",
        &mut &_ => (),
    }

    let file_content = format!(
        r#"export const AppDataSource = new DataSource({{
   type: "{db}",
   host: "localhost",
   port: 5473,
   username: "user",
   password: "password123",
   database: "dolph-app",
   entities: [],
   synchronize: process.env.NODE_ENV === "development",
   logging: true,
   migrations: [__dirname + "/migration/*.ts"], 
}});
"#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing datasource config file: {}", e));
            Err(Box::new(e))
        }
    }
}
