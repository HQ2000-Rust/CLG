use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

pub fn cp(file_path: PathBuf, new_file_path: PathBuf) -> std::io::Result<Option<String>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    //again, some more war crimes :D
    file.read_to_end(&mut contents)?;
    let new_file = File::create(new_file_path)?;
    let mut new_file_contents = BufWriter::new(new_file);
    new_file_contents.write_all(contents.as_slice())?;
    Ok(None)
}
