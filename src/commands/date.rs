use color_eyre::owo_colors::OwoColorize;

pub fn date() -> Option<String> {
    Some(chrono::Local::now().bold().to_string())
}
