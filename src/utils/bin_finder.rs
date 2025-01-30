use std::process::Command;

pub fn is_bin_installed(bin: &str) -> bool {
    match Command::new(String::from(bin)).arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}
