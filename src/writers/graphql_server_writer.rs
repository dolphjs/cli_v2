use std::{error::Error, fs};

use super::config_writers::find_base_directory;

pub fn write_graphql_server_file() -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let index_path = base_directory.join("server.ts");

    let import_statement = r#"import { DolphFactory} from "@dolphjs/dolph";
import { context, schema, session } from "./setup";
import { AppDataSource } from "./shared/configs/data_source";
import { logger } from "@dolphjs/dolph/utilities";
"#
    .to_string();

    let other_file_content = r#"
const dolph = new DolphFactory({ graphql: true, schema: schema(), context });

AppDataSource.initialize()
  .then(() => {
    dolph.start();
  })
  .catch((err) => logger.error(`[Dolph Error]:`, err));    
"#
    .to_string();

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing server file: {}", e));
            Err(Box::new(e))
        }
    }
}
