use std::fs::remove_dir;
use std::path::PathBuf;

pub fn rmdir(dirs: Vec<PathBuf>) -> std::io::Result<Option<String>> {
    for path in dirs {
        remove_dir(path)?;
    }
    Ok(None)
}
