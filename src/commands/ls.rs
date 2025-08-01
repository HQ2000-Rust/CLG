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
        .join("   ".into())
        .to_str()
        .expect("dirnames are expected to be displayable as UTF-8 for the entire tool to work")
        .to_string();
    Ok(Some(result))
}
