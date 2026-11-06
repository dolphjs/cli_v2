use std::{error::Error, fs};

use super::config_writers::find_base_directory;

// Databases connect through dolph_config.yaml, so this is the same for all of them.
pub fn write_graphql_server_file() -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let index_path = base_directory.join("server.ts");

    let file_content = r#"import { DolphFactory } from "@dolphjs/dolph";
import { context, schema } from "./setup";

const dolph = new DolphFactory([], { graphql: { schema: schema(), context } });

dolph.start();
"#;

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing server file: {}", e));
            Err(Box::new(e))
        }
    }
}
