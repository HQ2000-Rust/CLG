use mime_guess::from_path;
use mime_more::Mime;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn file(file_path: PathBuf) -> std::io::Result<Option<String>> {
    let mut file = File::open(&file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    if let Ok(mime) = Mime::from_content(&contents) {
        return Ok(Some(mime.to_string()));
    }
    if let Some(mime) = from_path(&Path::new(&file_path)).first() {
        return Ok(Some(mime.to_string()));
    }
    Ok(Some("unknown MIME type.".to_string()))
}
