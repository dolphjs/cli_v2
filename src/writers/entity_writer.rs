use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

pub fn write_entity(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory =
        find_base_directory().ok_or_else(|| "Could not find the base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let name_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&name_path)?;

    let entities_path = name_path.join("entities");

    ensure_directory_exists(&entities_path)?;

    let index_path = entities_path.join(format!("{}.entity.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = r#"import { Field, ID, ObjectType } from "type-graphql";
import { PrimaryGeneratedColumn } from "typeorm";"#
        .to_string();

    let other_file_content = format!(
        r#"@ObjectType()
@Entity()
@Index("{name}_index_0", ["id"])
export class {capitalized_name} {{
    @Field(() => ID)
    @PrimaryGeneratedColumn("uuid")
    id: string;
}}
"#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing entity file: {}", e));
            Err(Box::new(e))
        }
    }
}
