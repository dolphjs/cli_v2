use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

pub fn write_spring_component(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let specific_component_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&specific_component_path)?;

    let index_path = specific_component_path.join(format!("{}.component.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let content = format!(
        r#"import {{ Component }} from "@dolphjs/dolph/decorators";

@Component({{ controllers: [], services: [] }})
export class {capitalized_name}Component {{}};"#
    );

    match fs::write(&index_path, content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing component file: {}", e));
            Err(Box::new(e))
        }
    }
}
