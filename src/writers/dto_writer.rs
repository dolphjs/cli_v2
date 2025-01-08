use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

pub fn write_spring_dto(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let dto_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&dto_path)?;

    let index_path = dto_path.join(format!("{}.dto.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = r#"import {} from 'class-validator';
import {} from 'class-transformer';
    "#
    .to_string();

    let other_content = format!(r#"export class Create{capitalized_name}Dto {{}}"#);

    let file_content = format!("{}\n\n{}\n", import_statement, other_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing service file: {}", e));
            Err(Box::new(e))
        }
    }
}
