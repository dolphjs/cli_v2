use std::{
    path::PathBuf,
    process::{Child, Command},
};

use slog::Logger;

#[derive(Debug, Clone)]
pub struct CommandConfig {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub env: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct ServerProcess {
    pub child: Option<Child>,
    pub logger: Logger,
}

#[derive(Debug, Clone)]

pub struct WatchConfig {
    pub paths: Vec<PathBuf>,
    pub file_extensions: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub debounce_ms: u64,
}

#[derive(Debug)]
pub struct Config {
    pub command: CommandConfig,
    pub watch: WatchConfig,
}

impl CommandConfig {
    pub fn new(program: &str) -> Self {
        CommandConfig {
            program: program.to_string(),
            args: Vec::new(),
            cwd: None,
            env: Vec::new(),
        }
    }
}

impl ServerProcess {
    pub fn new(logger: Logger) -> Self {
        ServerProcess {
            child: None,
            logger,
        }
    }

    pub fn start(&mut self, config: &CommandConfig) {
        self.stop();

        slog::info!(self.logger, "Starting server: {}", config.program);

        let mut command = Command::new(&config.program);
        command.args(&config.args);

        if let Some(cwd) = &config.cwd {
            command.current_dir(cwd);
        }

        for (key, value) in &config.env {
            command.env(key, value);
        }

        match command.spawn() {
            Ok(child) => {
                self.child = Some(child);
                slog::info!(self.logger, "Server started successfully");
            }
            Err(e) => {
                slog::error!(self.logger, "Failed to start the server: {}", e);
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.child.take() {
            slog::info!(self.logger, "Stopping server ...");
            let _ = child.kill();
            let _ = child.wait();
            slog::info!(self.logger, "Server stopped");
        }
    }
}
