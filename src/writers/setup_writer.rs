use std::{error::Error, fs};

use super::config_writers::find_base_directory;

pub fn write_setup_file() -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let index_path = base_directory.join("setup.ts");

    let import_statement = format!(
        r#"import {{ buildSchema }} from "type-graphql";
        "#
    );

    let other_file_content = format!(
        r#"export const schema = async function createSchema() {{
return await buildSchema({{
    // Your resolvers should go here
    resolvers: [],
    validate: true,
    }});
}};

// Your context definition goes here
export const context = async ({{ req, res }}) => {{
  const session = req.session;
  const cookies = req.cookies;

  return {{ req, res, session, cookies }};
}};
    "#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing setup file: {}", e));
            Err(Box::new(e))
        }
    }
}
