use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

pub fn write_resolver(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory =
        find_base_directory().ok_or_else(|| "Could not find the base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let name_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&name_path)?;

    let resolver_path = name_path.join("resolvers");

    ensure_directory_exists(&resolver_path)?;

    let index_path = resolver_path.join(format!("{}.resolver.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = r#"import { Mutation, Query, Resolver } from "type-graphql";
    "#
    .to_string();

    let other_file_content = format!(
        r#"@Resolver()
export class {capitalized_name}Resolver{{
    // You should place you service here

    constructor(){{
        // You should instantiate the service here
    }}

    @Mutation()
    async create{capitalized_name}() {{
        
    }}

    @Query()
    async fetch{capitalized_name}() {{}}
}}
"#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing resolver file: {}", e));
            Err(Box::new(e))
        }
    }
}
