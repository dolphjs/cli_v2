use std::error::Error;

use crate::utils::{capitalize_first_letter, ensure_directory_exists, write_generated};

use super::config_writers::{find_base_directory, Database};

pub fn write_spring_model(db: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let model_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&model_path)?;

    let capitalized_name = capitalize_first_letter(name);

    let database = Database::from_str(db).ok_or_else(|| "Invalid database type")?;

    // SQL models are TypeORM entities. The `.entity` suffix is what the
    // `entities` glob in dolph_config.yaml looks for.
    let (file_name, import_statement, other_file_content) = match database {
        Database::MySQL | Database::PostgreSQL => (
            format!("{}.entity.ts", name),
            r#"import { Entity, PrimaryGeneratedColumn } from "typeorm";"#.to_string(),
            format!(
                r#"@Entity()
export class {capitalized_name} {{
    @PrimaryGeneratedColumn("uuid")
    id: string;
}}"#
            ),
        ),
        Database::MongoDB => (
            format!("{}.model.ts", name),
            r#"import { Schema, Document, model } from "mongoose";"#.to_string(),
            format!(
                r#"export interface I{capitalized_name} extends Document {{}};
 
 const {capitalized_name}Schema = new Schema(
    {{

    }});

export const {capitalized_name}Model = model<I{capitalized_name}>("{name}", {capitalized_name}Schema);
 "#
            ),
        ),
        Database::None => (format!("{}.model.ts", name), String::new(), String::new()),
    };

    let index_path = model_path.join(file_name);

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match write_generated(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing model file: {}", e));
            Err(Box::new(e))
        }
    }
}
