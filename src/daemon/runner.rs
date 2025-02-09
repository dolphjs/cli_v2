use slog;
use std::{path::PathBuf, process::Command};

use crate::daemon::watcher::setup_logger;

pub fn runner(env: &str, port: &str, language: &str, bun: bool) {
    let logger = setup_logger();
    slog::info!(logger, "Starting server");

    let (bin, args) = if bun {
        ("bun", vec!["run", "./app/src/server.js"])
    } else if language == "ts" {
        ("ts-node", vec!["./app/src/server.js"])
    } else {
        ("node", vec!["./app/src/server.js"])
    };

    let mut command = Command::new(bin);
    command
        .args(args)
        .current_dir(".")
        .env("NODE_ENV", env)
        .env("PORT", port);

    match command.status() {
        Ok(status) => {
            if status.success() {
                slog::info!(logger, "Server started successfully");
            } else {
                slog::error!(logger, "Server exited with status: {}", status);
            }
        }
        Err(e) => slog::error!(logger, "Failed to start server: {}", e),
    }

    slog::info!(logger, "Server process ended");
}
