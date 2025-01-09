use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::{find_base_directory, Database};

pub fn write_spring_service(db: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let service_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&service_path)?;

    let index_path = service_path.join(format!("{}.service.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let database = Database::from_str(db).ok_or_else(|| "Invalid database type")?;

    let import_statement = match database.clone() {
        Database::MySQL => format!(
            r#"import {{ DolphServiceHandler }} from "@dolphjs/dolph/classes";
import {{ Dolph }} from "@dolphjs/dolph/common";
import {{ InjectMySQL }} from "@dolphjs/dolph/decorators";
import {{ ModelStatic, Model }} from "sequelize";
import {{ {name}Model }} from "./{name}.model"; "#
        ),
        Database::MongoDB => format!(
            r#"import {{ DolphServiceHandler }} from "@dolphjs/dolph/classes";
import {{ Dolph }} from "@dolphjs/dolph/common";
import {{ InjectMongo }} from "@dolphjs/dolph/decorators";
import {{ Model }} from "mongoose";
import {{ {capitalized_name}Model, I{capitalized_name} }} from "./{name}.model"; "#
        ),
        Database::None => format!(
            r#"import {{ DolphServiceHandler }} from "@dolphjs/dolph/classes";
import {{ Dolph }} from "@dolphjs/dolph/common";"#
        ),
    };

    let other_file_content = match database.clone() {
        Database::MongoDB => format!(
            r#"@InjectMongo("{name}Model", {capitalized_name}Model)
export class {capitalized_name}Service extends DolphServiceHandler<Dolph>{{
    private {name}Model!: Model<I{capitalized_name}>;

    constructor() {{
        super("{name}Service");
    }}
}}"#
        ),
        Database::MySQL => format!(
            r#"@InjectMySQL("{name}Model", )
export class {capitalized_name}Service extends DolphServiceHandler<Dolph>{{
    private {name}Model!: ModelStatic<Model<any, any>>;

    constructor() {{
        super("{name}Service");
    }}
}}"#
        ),
        Database::None => format!(
            r#"export class {capitalized_name}Service extends DolphServiceHandler<Dolph>{{
    constructor() {{
        super("{name}Service");
    }}
}}"#
        ),
    };

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing service file: {}", e));
            Err(Box::new(e))
        }
    }
}

pub fn write_graphql_service(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let name_directory = component_path.join(format!("{}", name));

    ensure_directory_exists(&name_directory)?;

    let service_path = name_directory.join("services");

    ensure_directory_exists(&service_path)?;

    let index_path = service_path.join(format!("{}.service.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = format!(
        r#"import {{ DolphServiceHandler }} from "@dolphjs/dolph/classes";
import {{ Dolph }} from "@dolphjs/dolph/common";
import {{ Create{capitalized_name}Input }} from "../inputs/{name}.inputs";
"#
    );

    let other_file_content = format!(
        r#"export class {capitalized_name}Service extends DolphServiceHandler<Dolph> {{
// Your repository should be here

    constructor() {{
        super("{name}Service");
        // Your repository should be initialized here
    }}

    async create{capitalized_name}(data: Create{capitalized_name}Input) {{}}
}}
"#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing service file: {}", e));
            Err(Box::new(e))
        }
    }
}
