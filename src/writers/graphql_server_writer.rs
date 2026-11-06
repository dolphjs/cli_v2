use std::{error::Error, fs};

use super::config_writers::find_base_directory;

pub fn write_graphql_server_file(database: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let index_path = base_directory.join("server.ts");

    // Mongo connects through dolph_config.yaml. A TypeORM DataSource is only
    // generated for SQL databases.
    let (import_statement, other_file_content) = if database == "mongo" {
        (
            r#"import { DolphFactory } from "@dolphjs/dolph";
import { context, schema } from "./setup";
"#,
            r#"
const dolph = new DolphFactory({ graphql: true, schema: schema(), context });

dolph.start();
"#,
        )
    } else {
        (
            r#"import { DolphFactory, logger } from "@dolphjs/dolph";
import { context, schema } from "./setup";
import { AppDataSource } from "./shared/configs/data_source";
"#,
            r#"
const dolph = new DolphFactory({ graphql: true, schema: schema(), context });

AppDataSource.initialize()
  .then(() => {
    dolph.start();
  })
  .catch((err) => logger.error(`[Dolph Error]:`, err));
"#,
        )
    };

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing server file: {}", e));
            Err(Box::new(e))
        }
    }
}
