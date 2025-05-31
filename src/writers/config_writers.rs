use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::utils::capitalize_first_letter;

#[derive(Debug, Clone, Copy)]
pub enum Database {
    MySQL,
    MongoDB,
    PostgreSQL,
    None,
}

impl Database {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mysql" => Some(Database::MySQL),
            "mongo" => Some(Database::MongoDB),
            "other" => Some(Database::PostgreSQL),
            "postgresql" => Some(Database::None),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DefaultDolphConfig {
    pub jsonLimit: String,
    pub port: String,
}

pub fn find_base_directory() -> Option<PathBuf> {
    let root_dir = std::env::current_dir().ok()?;
    let possible_dirs = vec!["src"];

    for dir in possible_dirs {
        let path = root_dir.join(dir);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

pub fn write_spring_server_file(db: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let index_path = base_directory.join("server.ts");
    let database = Database::from_str(db).ok_or_else(|| "Invalid database type")?;

    let capitalized_name = capitalize_first_letter(name);

    // Generate import statement
    let import_statement = match (database.clone(), name.is_empty()) {
        (Database::MySQL, false) => format!(r#"import {{ DolphFactory }} from "@dolphjs/dolph";"#),
        (Database::MySQL, true) => r#"import { DolphFactory } from "@dolphjs/dolph";
import { sequelizeInstance } from "@/shared/configs/db.config";
import { autoInitMySql } from "@dolphjs/dolph/packages";"#
            .to_string(),
        (Database::MongoDB, false) => {
            format!(r#"import {{ DolphFactory }} from "@dolphjs/dolph";"#)
        }
        (Database::None, false) => format!(r#"import {{ DolphFactory }} from "@dolphjs/dolph";"#),
        (Database::None, true) => format!(
            r#"import {{ DolphFactory }} from "@dolphjs/dolph";
import {{ {capitalized_name}Component }} from "./components/{name}/{name}.component.ts";"#
        ),
        (Database::PostgreSQL, false) => {
            format!(r#"import {{ DolphFactory }} from "@dolphjs/dolph";"#)
        }
        (Database::PostgreSQL, true) => format!(
            r#"import {{ DolphFactory }} from "@dolphjs/dolph";
import {{ {capitalized_name}Component }} from "./components/{name}/{name}.component.ts";"#
        ),
        (Database::MongoDB, true) => {
            r#"import { DolphFactory } from "@dolphjs/dolph";"#.to_string()
        }
    };

    // Generate other file content
    let other_file_content = match (database.clone(), name.is_empty()) {
        (Database::MySQL, false) => format!(
            r#"const dolph = new DolphFactory([{capitalized_name}Component]);
autoInitMySql(sequelizeInstance);

dolph.start();"#
        ),
        (Database::MySQL, true) => r#"const dolph = new DolphFactory([]);
autoInitMySql(sequelizeInstance);

dolph.start();"#
            .to_string(),
        (Database::MongoDB, false) => format!(
            r#"const dolph = new DolphFactory([{capitalized_name}Component]);
dolph.start();"#
        ),
        (Database::None, false) => format!(
            r#"const dolph = new DolphFactory([{capitalized_name}Component]);
dolph.start();"#
        ),
        (Database::None, true) => format!(
            r#"const dolph = new DolphFactory([{capitalized_name}Component]);
dolph.start();"#
        ),
        (Database::PostgreSQL, false) => format!(
            r#"const dolph = new DolphFactory([{capitalized_name}Component]);
dolph.start();"#
        ),
        (Database::PostgreSQL, true) => format!(
            r#"const dolph = new DolphFactory([{capitalized_name}Component]);
dolph.start();"#
        ),
        (Database::MongoDB, true) => r#"const dolph = new DolphFactory([]);
dolph.start();"#
            .to_string(),
    };

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing server file: {}", e));
            Err(Box::new(e))
        }
    }
}

pub fn write_swcrc(is_spring: bool) -> Result<(), Box<dyn Error>> {
    // Implementation for writing .swcrc file
    let root_dir = get_root_directory()?;
    let file_path = root_dir.join(".swcrc");

    let paths = if !is_spring {
        json!({
            "@/*": ["*"],
            "@/configs/*": ["configs/*"],
            "@/controllers/*": ["controllers/*"],
            "@/dtos/*": ["dtos/*"],
            "@/interfaces/*": ["interfaces/*"],
            "@/middlewares/*": ["middlewares/*"],
            "@/models/*": ["models/*"],
            "@/routes/*": ["routes/*"],
            "@/services/*": ["services/*"],
            "@/utils/*": ["utils/*"],
            "@/constants/*": ["constants/*"],
            "@/validations/*": ["validations/*"]
        })
    } else {
        json!({
            "@/*": ["*"],
            "@/configs/*": ["shared/configs/*"],
            "@/components/*": ["components/*"],
            "@/utils/*": ["shared/utils/*"],
            "@/shields/*": ["shared/shields/*"],
            "@/shared/*": ["shared/*"],
            "@/helpers*": ["shared/helpers/*"],
            "@/interfaces/*": ["shared/interfaces/*"],
            "@/middlewares/*": ["shared/middlewares/*"],
            "@/decorators/*": ["shared/decorators/*"],
            "@/services/*": ["shared/services/*"],
            "@/constants/*": ["shared/constants/*"],
            "@/validations/*": ["shared/validations/*"]
        })
    };

    let config = json!({
        "jsc": {
            "parser": {
                "syntax": "typescript",
                "tsx": false,
                "dynamicImport": true,
                "decorators": true
            },
            "transform": {
                "legacyDecorator": true,
                "decoratorMetadata": true
            },
            "target": "es2022",
            "externalHelpers": false,
            "keepClassNames": true,
            "loose": false,
            "minify": {
                "compress": false,
                "mangle": false
            },
            "baseUrl": "src",
            "paths": paths
        },
        "module": {
            "type": "commonjs"
        }
    });

    // Pretty print the JSON with proper indentation
    let config_str = serde_json::to_string_pretty(&config)?;

    fs::write(file_path, config_str)?;

    Ok(())
}

pub fn write_tsconfig(is_spring: bool) -> Result<(), Box<dyn Error>> {
    // Implementation for writing tsconfig.json
    let root_dir = get_root_directory()?;
    let file_path = root_dir.join("tsconfig.json");

    let paths = if !is_spring {
        json!({
          "@/*": ["*"],
          "@/configs/*": ["configs/*"],
          "@/controllers/*": ["controllers/*"],
          "@/dtos/*": ["dtos/*"],
          "@/interfaces/*": ["interfaces/*"],
          "@/middlewares/*": ["middlewares/*"],
          "@/models/*": ["models/*"],
          "@/routes/*": ["routes/*"],
          "@/services/*": ["services/*"],
          "@/utils/*": ["utils/*"],
          "@/constants/*": ["constants/*"],
        })
    } else {
        json!({
          "@/*": ["*"],
          "@/configs/*": ["shared/configs/*"],
          "@/components/*": ["components/*"],
          "@/utils/*": ["shared/utils/*"],
          "@/shields/*": ["shared/shields/*"],
          "@/shared/*": ["shared/*"],
          "@/helpers*": ["shared/helpers/*"],
          "@/interfaces/*": ["shared/interfaces/*"],
          "@/middlewares/*": ["shared/middlewares/*"],
          "@/decorators/*": ["shared/decorators/*"],
          "@/services/*": ["shared/services/*"],
          "@/constants/*": ["shared/constants/*"],
          "@/validations/*": ["shared/validations/*"]
        })
    };

    let config = json!({
      "exclude": ["node_modules"],
      "compilerOptions": {
        "allowJs": false,
        "declaration": false,
        "experimentalDecorators": true,
        "emitDecoratorMetadata": true,
        "sourceMap": false,
        "useUnknownInCatchVariables": false,
        "strictFunctionTypes": false,
        "forceConsistentCasingInFileNames": true,
        "resolveJsonModule": true,
        "esModuleInterop": true,
        "target": "ES2022",
        "baseUrl": "src",
        "outDir": "app",
        "pretty": true,
        "module": "commonjs",
        "paths": paths,
      },
      "include": ["src/", ".env"],
      "ts-node": {
        "compilerOptions": {
          "module": "CommonJS"
        }
      }
    });

    // Pretty print the JSON with proper indentation
    let config_str = serde_json::to_string_pretty(&config)?;
    fs::write(file_path, config_str)?;

    Ok(())
}

pub fn write_dolph_config() -> Result<(), Box<dyn Error>> {
    // Implementation for writing dolph config
    let root_dir = get_root_directory()?;
    let file_path = root_dir.join("dolph_config.yaml");

    let config = DefaultDolphConfig {
        jsonLimit: String::from("2mb"),
        port: String::from("3300"),
    };

    // Pretty print the JSON with proper indentation
    let config_str = serde_yaml::to_string(&config)?;
    fs::write(file_path, config_str)?;
    Ok(())
}

pub fn write_package_json(
    project_name: &str,
    language: &str,
    api: &str,
) -> Result<(), Box<dyn Error>> {
    // Implementation for writing package.json
    let root_dir = get_root_directory()?;
    let file_path = root_dir.join("package.json");

    let config = if language.to_string() == "ts" && api.to_string() == "rest" {
        json!({
          "name": project_name.to_string(),
          "version": "1.0.0",
          "main": "app/server.js",
          "author": "",
          "license": "MIT",
          "engines": {
            "node": ">=18.0.0"
          },
          "scripts": {
            "dev:start": "dolph watch",
            "dev:docker:start": "docker-compose -f docker-compose-dev.yml up",
            "dev:docker:stop": "docker-compose -f docker-compose-dev.yml down",
            "build": "dolph build",
            "build:tsc": "tsc && tsc-alias",
            "start": "dolph start",
            "clean": "rm -r app && rm -r logs"
          },
          "dependencies": {
            "@dolphjs/dolph": "^2.0.1"
          },
          "devDependencies": {
            "@swc/cli": "^0.1.62",
            "@swc/core": "^1.3.91",
             "@types/express": "^4.17.21",
            "@types/node": "^20.8.2",
            "ts-node": "^10.9.1",
            "tsc-alias": "^1.8.8",
            "tsconfig-paths": "^4.2.0",
            "typescript": "^5.2.2"
          }
        })
    } else if api.to_string() == "graphql" {
        json!({
         "name": project_name.to_string(),
          "version": "1.0.0",
          "main": "app/server.js",
          "author": "",
          "license": "MIT",
          "engines": {
            "node": ">=18.0.0"
          },
          "scripts": {
            "dev:start": "dolph watch",
            "dev:docker:start": "docker-compose -f docker-compose-dev.yml up",
            "dev:docker:stop": "docker-compose -f docker-compose-dev.yml down",
            "build": "dolph build",
            "build:tsc": "tsc && tsc-alias",
            "start": "dolph start",
            "clean": "rm -r app && rm -r logs"
          },
          "dependencies": {
            "@dolphjs/dolph": "^2.0.1",
            "@dolphjs/graphql": "^0.2.0",
            "graphql-scalars": "^1.23.0",
            "type-graphql": "^2.0.0-rc.2",
            "typeorm": "^0.3.20"
          },
          "devDependencies": {
            "@swc/cli": "^0.1.62",
            "@swc/core": "^1.3.91",
             "@types/express": "^4.17.21",
            "@types/node": "^20.8.2",
            "ts-node": "^10.9.1",
            "tsc-alias": "^1.8.8",
            "tsconfig-paths": "^4.2.0",
            "typescript": "^5.2.2"
          }
        })
    } else {
        json!({
          "name": project_name.to_string(),
          "version": "1.0.0",
          "main": "src/server.js",
          "author": "",
          "license": "MIT",
          "engines": {
            "node": ">=18.0.0"
          },
          "scripts": {
            "dev:start": "dolph watch",
            "dev:docker:start": "docker-compose -f docker-compose-dev.yml up",
            "dev:docker:stop": "docker-compose -f docker-compose-dev.yml down",
            "clean": "rm -r logs",
            "start": "dolph start"
          },
          "dependencies": {
            "@dolphjs/dolph": "^2.0.1"
          }
        })
    };

    // Pretty print the JSON with proper indentation
    let config_str = serde_json::to_string_pretty(&config)?;
    fs::write(file_path, config_str)?;
    Ok(())
}

pub fn write_gitignore() -> Result<(), Box<dyn Error>> {
    // Implementation for writing .gitignore
    let root_dir = get_root_directory()?;
    let file_path = root_dir.join(".gitignore");

    let config = r#"node_modules
.env
logs
yarn-error.log
app"#
        .to_string();

    let file_content = format!("{}", config);

    match fs::write(&file_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing server file: {}", e));
            Err(Box::new(e))
        }
    }
}

fn get_root_directory() -> Result<PathBuf, Box<dyn Error>> {
    Ok(std::env::current_dir()?)
}
