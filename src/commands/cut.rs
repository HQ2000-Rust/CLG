use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn cut(
    file_path: PathBuf,
    start: Option<u32>,
    end: Option<u32>,
) -> std::io::Result<Option<String>> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let mut char_in_line = 0u32;
    let start = start.expect("Optional, but it has a default value so it's always Some(_)");
    //for the algo, again
    let mut result = String::new();
    content.push('\n');
    for char in content.chars() {
        match char {
            '\n' => {
                result.push('\n');
                dbg!("reset");
                char_in_line = 0;
            }
            '\t' => continue,
            _ => {
                char_in_line += 1;
                if let Some(end_pos) = end
                    && char_in_line > end_pos
                {
                    continue;
                }
                if char_in_line >= start {
                    result.push(char);
                }
            }
        }
    }
    Ok(Some(result))
}
