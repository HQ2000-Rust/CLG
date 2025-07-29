mod parsing;
mod gambling;
mod commands;

use clap::Parser;
use command_processing::Cli;

fn main() {
    let cli = Cli::parse();
    println!("command: {:?}", cli.command);
}
