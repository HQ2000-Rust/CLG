use std::fs::create_dir;

pub fn mkdir(names: Vec<String>) -> std::io::Result<Option<String>> {
    for name in names {
        create_dir(name)?;
    }
    Ok(None)
}
