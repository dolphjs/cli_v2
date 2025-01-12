use std::error::Error;
use tokio::process::Command as AsyncCommand;

pub async fn build_ts_app(language: &str) -> Result<(), Box<dyn Error>> {
    if language.to_string() != "ts" {
        println!(
            "{} {}",
            "[Dolph Error]: ", "Cannot compile javascript file, exciting compilation..."
        );
        return Err("Cannot compile Javascript file".into());
    }

    println!(
        "{} {}",
        "[Dolph Info]: ", "Compiling from Typescript to Javascript..."
    );

    let spawn_args = vec!["src", "-d", "app", "--source-maps", "--copy-files"];

    let mut child = AsyncCommand::new("swc").args(&spawn_args).spawn()?;

    match child.wait().await {
        Ok(status) => {
            if !status.success() {
                println!("{} {}", "[Dolph Error]: ", "Exciting compilation...");
                return Err("Compilation error...".into());
            }

            println!("{} {}", "[Dolph Info]: ", "Compilation successful");
            Ok(())
        }
        Err(e) => {
            println!("{} {}", "[Dolph Error]: ", format!("{}", e));
            return Err(e.into());
        }
    }
}
