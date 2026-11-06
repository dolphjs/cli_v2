use std::{error::Error, fs};

use crate::utils::ensure_directory_exists;

use super::config_writers::find_base_directory;

pub fn write_db_config(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let shared_path = base_directory.join("shared");

    ensure_directory_exists(&shared_path)?;

    let configs_path = shared_path.join("configs");

    ensure_directory_exists(&configs_path)?;

    let index_path = configs_path.join("db.config.ts");

    let import_statement = r#"import { initSql } from "@dolphjs/dolph";"#.to_string();

    let file_content = r#"export const sequelizeInstance = initSql({
  dialect: "mysql",
  database: process.env.SQL_DATABASE ?? "dolph",
  user: process.env.SQL_USER,
  pass: process.env.SQL_PASSWORD,
  host: process.env.SQL_HOST ?? "localhost",
});"#
    .to_string();

    let file_content = format!("{}\n\n{}\n", import_statement, file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing db config file: {}", e));
            Err(Box::new(e))
        }
    }
}
