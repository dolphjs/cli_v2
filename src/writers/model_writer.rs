use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::{find_base_directory, Database};

pub fn write_spring_model(db: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let model_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&model_path)?;

    let index_path = model_path.join(format!("{}.model.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let database = Database::from_str(db).ok_or_else(|| "Invalid database type")?;

    let import_statement = match database.clone() {
        Database::MySQL => format!(
            r#"import {{ sequelizeInstance }} from "@/shared/configs/db.config";
import {{ DataTypes }} from "sequelize";"#
        ),
        Database::MongoDB => format!(r#"import {{ Schema, Document, model }} from "mongoose";"#),
        Database::None => r#""#.to_string(),
    };

    let other_file_content = match database.clone() {
        Database::MySQL => format!(
            r#"export const {capitalized_name} = sequelizeInstance.define("{name}", {{
  id: {{
    type: DataTypes.INTEGER,
    allowNull: false,
    primaryKey: true,
    autoIncrement: true,
    }},
}});"#
        ),
        Database::MongoDB => format!(
            r#"export interface I{capitalized_name} extends Document {{}};
 
 const {capitalized_name}Schema = new Schema(
    {{

    }});

export const {capitalized_name}Model = model<I{capitalized_name}>("{name}", {capitalized_name}Schema);
 "#
        ),
        Database::None => r#""#.to_string(),
    };

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing model file: {}", e));
            Err(Box::new(e))
        }
    }
}
