use std::env::current_dir;

pub fn dirname() -> std::io::Result<Option<String>> {
    Ok(Some(match current_dir()?.file_name() {
        None => "..".to_string(),
        Some(name) => name.to_string_lossy().to_string(),
    }))
}
