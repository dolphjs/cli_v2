use std::{
    fs, io,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

// Set once from `--force`. A flag here means no generator can forget to honour it.
static OVERWRITE: AtomicBool = AtomicBool::new(false);
static SKIPPED: AtomicBool = AtomicBool::new(false);

pub fn set_overwrite(overwrite: bool) {
    OVERWRITE.store(overwrite, Ordering::Relaxed);
}

/// Whether a file was skipped since the last call, so a command doesn't claim to
/// have generated something it left alone.
pub fn take_skipped() -> bool {
    SKIPPED.swap(false, Ordering::Relaxed)
}

fn should_write(path: &Path, overwrite: bool) -> bool {
    overwrite || !path.exists()
}

/// Writes a file for `dolph generate`. These files are the user's once they exist,
/// so one that's already there is kept unless `--force` was passed.
pub fn write_generated(path: &Path, content: impl AsRef<[u8]>) -> io::Result<()> {
    if should_write(path, OVERWRITE.load(Ordering::Relaxed)) {
        return fs::write(path, content);
    }

    let shown = std::env::current_dir()
        .ok()
        .and_then(|root| path.strip_prefix(root).ok().map(Path::to_path_buf))
        .unwrap_or_else(|| path.to_path_buf());

    println!("Skipped {}: it already exists. Use --force to replace it.", shown.display());
    SKIPPED.store(true, Ordering::Relaxed);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_a_file_that_does_not_exist() {
        assert!(should_write(Path::new("/dolph/does/not/exist.ts"), false));
    }

    #[test]
    fn should_keep_an_existing_file() {
        assert!(!should_write(Path::new(file!()).parent().unwrap().parent().unwrap(), false));
    }

    #[test]
    fn should_replace_an_existing_file_with_force() {
        assert!(should_write(Path::new(file!()).parent().unwrap().parent().unwrap(), true));
    }
}
