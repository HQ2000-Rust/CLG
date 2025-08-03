use std::fs::remove_file;
use std::path::PathBuf;

pub fn rm(files: Vec<PathBuf>) -> std::io::Result<Option<String>> {
    //IMPORTANT: this command removes every file until it hits an error or the list ends
    for path in files {
        remove_file(path)?;
    }
    Ok(None)
}
