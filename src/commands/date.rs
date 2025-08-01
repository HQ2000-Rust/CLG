pub fn date() -> Option<String> {
    Some(format!("{}", chrono::Local::now()))
}
