use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

pub fn write_input(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let name_directory = component_path.join(format!("{}", name));

    ensure_directory_exists(&name_directory)?;

    let input_path = name_directory.join("inputs");

    ensure_directory_exists(&input_path)?;

    let index_path = input_path.join(format!("{}.input.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = format!(
        r#"import {{}} from "class-validator";
import {{ Field, InputType }} from "type-graphql";"#
    );

    let other_file_content = format!(
        r#"@InputType()
export class Create{capitalized_name}Input{{}}
"#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing input file: {}", e));
            Err(Box::new(e))
        }
    }
}
