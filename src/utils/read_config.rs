use std::{error::Error, fs, path::Path};

use crate::properties::DefaultConfig;

pub fn read_config() -> Result<DefaultConfig, Box<dyn Error>> {
    // Todo: return to this before pushing to NPM
    // let config_path = Path::new("dolph_cli.yaml");

    let app_path = Path::new("app");
    let config_path = app_path.join("dolph_cli.yaml");

    let yaml_content = fs::read_to_string(config_path)?;

    let config_content = yaml_content
        .lines()
        .filter(|line| !line.trim_start().starts_with("#"))
        .collect::<Vec<&str>>()
        .join("\n");

    let config: DefaultConfig = serde_yaml::from_str(&config_content)?;

    Ok(config)
}

pub fn ensure_directory_exists(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}
