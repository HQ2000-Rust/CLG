use std::fmt::Error;
use std::fs::{File, read_dir};
use std::io::{BufReader, ErrorKind, Read};
use std::path::{Path, PathBuf};

pub fn cat(file_paths: Vec<PathBuf>) -> Result<Option<String>, std::io::Error> {
    if file_paths.is_empty() {
        return Ok(Some("No files specified!".to_string()));
    }
    let mut result = String::new();
    for file_path in file_paths {
        let file = File::open(file_path.clone())?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let file_name = file_path
            .file_name()
            .expect("File name needs to be a valid string")
            .to_string_lossy()
            .into_owned();
        result.push_str(&[file_name, content].join("\n"));
        drop(reader);
    }
    Ok(Some(result))
}
