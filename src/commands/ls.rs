use clap;
use std::ffi::OsString;
use std::fs::{DirEntry, read_dir};
use std::path::Path;

pub fn ls() -> Result<Option<String>, std::io::Error> {
    let mut result = Vec::new();
    for entry in read_dir(Path::new("."))? {
        let entry: DirEntry = entry?;
        result.push(entry.file_name());
    }
    let result = result
        .iter()
        .map(|os_string| os_string.to_string_lossy())
        .collect::<Vec<_>>()
        .join("   ");
    Ok(Some(result))
}
