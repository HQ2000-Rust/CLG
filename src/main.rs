mod command_processing;
mod gambling;

use clap::Parser;
use command_processing::Cli;

fn main() {
    let cli = Cli::parse();
    println!("command: {:?}", cli.command);
}
