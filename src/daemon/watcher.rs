use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
    vec,
};

use notify::{
    Config as NotifyConfig, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
#[cfg(unix)]
use signal_hook::{consts::SIGINT, iterator::Signals};

use slog::{o, Drain, Logger};

use crate::daemon::configs::{CommandConfig, Config, ServerProcess, WatchConfig};

fn setup_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")))
}

fn should_ignore(path: &Path, ignore_patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignore_patterns.iter().any(|pattern| {
        glob::Pattern::new(pattern)
            .map(|p| p.matches(&path_str))
            .unwrap_or(false)
    })
}

fn should_restart_for_event(event: &Event, config: &WatchConfig) -> bool {
    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) => {}
        _ => return false,
    }

    for path in &event.paths {
        if should_ignore(&path, &config.ignore_patterns) {
            continue;
        }

        if let Some(extension) = path.extension() {
            if config
                .file_extensions
                .contains(&extension.to_string_lossy().to_string())
            {
                return true;
            }
        }
    }
    false
}

pub fn watcher(env: &str, port: &str, language: &str) {
    let logger = setup_logger();
    slog::info!(logger, "Starting dolph server daemon");

    let mut bin = "node";
    let mut server_path = "./src/server.js";

    if language.to_string() == "ts" {
        bin = "ts-node"; // update to ts-node-dev
        server_path = "./src/server.ts";
    };

    let config = Config {
        command: CommandConfig {
            program: String::from(bin),
            args: vec![String::from(server_path)],
            cwd: Some(PathBuf::from(".")),
            env: vec![
                ("NODE_ENV".to_string(), env.to_string()),
                ("PORT".to_string(), port.to_string()),
            ],
        },
        watch: WatchConfig {
            paths: vec![PathBuf::from("./src")],
            file_extensions: vec![String::from("ts"), String::from("js"), String::from("json")],
            ignore_patterns: vec![
                String::from("**/node_modules/**"),
                String::from("**/.git/**"),
                // String::from("**/app/**"),
                String::from("**/.#*"),
                String::from("**/*~"),
                String::from("**/*.swp"),
                String::from("**/*.swx"),
                String::from("**/.DS_Store"),
            ],
            debounce_ms: 500,
        },
    };

    let running = Arc::new(AtomicBool::new(true));
    let config = Arc::new(config);
    let last_restart = Arc::new(Mutex::new(std::time::Instant::now()));
    let logger_clone = logger.clone();
    let running_clone = running.clone();

    // Setting up ctrl+c handler for windows
    if cfg!(windows) {
        ctrlc::set_handler(move || {
            slog::info!(logger_clone, "Received Ctrl+c signal");
            running_clone.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl+c handler")
    }

    #[cfg(unix)]
    {
        let running_clone = running.clone();
        let logger_signals = logger.clone();
        let mut signals = Signals::new(&[SIGINT]).unwrap();

        thread::spawn(move || {
            for sig in signals.forever() {
                slog::info!(logger_signals, "Received signal: {}", sig);
                running_clone.store(false, Ordering::SeqCst);
                break;
            }
        });
    }

    // Create server process manager
    let mut server = ServerProcess::new(logger.clone());

    server.start(&config.command);

    // Setup file watcher
    let (tx, rx) = std::sync::mpsc::channel();

    let watcher_config =
        NotifyConfig::default().with_poll_interval(Duration::from_millis(config.watch.debounce_ms));

    let mut watcher: RecommendedWatcher = Watcher::new(tx, watcher_config).unwrap();

    for watch_path in &config.watch.paths {
        match watcher.watch(&watch_path, RecursiveMode::Recursive) {
            Ok(_) => slog::info!(logger, "Watching path {:?} for changes", watch_path),
            Err(e) => slog::error!(logger, "Failed to watch path: {:?} : {}", watch_path, e),
        }
    }

    let config_clone = config.clone();

    while running.load(Ordering::SeqCst) {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                if let Ok(event) = event {
                    if should_restart_for_event(&event, &config_clone.watch) {
                        let now = std::time::Instant::now();
                        let mut last = last_restart.lock().unwrap();

                        if now.duration_since(*last).as_millis()
                            > config_clone.watch.debounce_ms as u128
                        {
                            slog::info!(logger, "Change detected in {:?}", event.paths[0]);

                            slog::info!(logger, "Restarting server...");

                            thread::sleep(Duration::from_millis(100));
                            server.start(&config_clone.command);
                            *last = now;
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(e) => slog::error!(logger, "Watch error: {:?}", e),
        }
    }

    server.stop();
    slog::info!(logger, "Dolph watch daemon stopped");
}
