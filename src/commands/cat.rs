use crossterm::style::Stylize;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

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
            .expect("not a valid path")
            .to_string_lossy()
            .into_owned();
        //ugly, but I want the "header" to be bold
        result.push_str(
            &[
                format!("Contents of {}:", file_name).bold().to_string(),
                format!("\n{}\n", content),
            ]
            .join(""),
        );
    }
    Ok(Some(result))
}
