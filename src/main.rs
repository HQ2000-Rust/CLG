mod parsing;
mod gambling;
mod commands;

use clap::Parser;
use parsing::Cli;

fn main() {
    let cli = Cli::parse();
    println!("command: {:?}", cli.command);
}
