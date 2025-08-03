mod commands;
mod gambling;
mod parsing;
mod run;

use crate::run::run;
use clap::Parser;
use parsing::Cli;
use ratatui::crossterm::style::Stylize;

fn main() {
    let cli = Cli::parse();
    //the run function basically does everything
    match run(cli) {
        Err(e) => {
            eprintln!("{}: {}", "error".red().bold(), e.to_string().bold(),);
        }
        Ok(_) => {}
    }
}
