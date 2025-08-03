use super::{date, dirname};

pub fn echo(name: String) -> std::io::Result<Option<String>> {
    //maybe I'll add more "variables" soon, just have no idea which ones
    let mut result = name
        .replace("\\n", "\n")
        .replace("\\r", "\r")
        .replace("\\t", "\t")
        .replace(
            "!DATE",
            date()
                .expect("Alwas returning Some(_)")
                .to_string()
                .as_str(),
        )
        .replace(
            "!DIRNAME",
            dirname()?.expect("Always returning Some(_)").as_str(),
        );
    result.push_str("\n");
    Ok(Some(result))
}
