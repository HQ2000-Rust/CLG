use crate::commands::{
    cat, cp, cut, date, dirname, echo, file, getconf, ls, mkdir, rm, rmdir, sleep, wc,
};
use crate::parsing::{Cli, Commands};
use crossterm::style::Stylize;

pub fn run(cli: Cli) -> std::io::Result<()> {
    let cmd = match cli.command {
        None => {
            print!("{}", "Try --help :)".bold());
            std::process::exit(0);
        }
        Some(cmd) => cmd,
    };
    //own function later, so I can handle the errors here
    let gambling_result = crate::gambling::rand_gamble_with_difficulty(
        cli.difficulty
            .expect("Optional, but it has a default value"),
    );

    if !gambling_result.is_failure() {
        let output = match cmd {
            Commands::Cat { files } => cat(files)?,
            Commands::Cp { file, new_file } => cp(file, new_file)?,
            Commands::Cut { file, start, end } => cut(file, start, end)?,
            Commands::Date => date(),
            Commands::Dirname => dirname()?,
            Commands::Echo { name } => echo(name)?,
            Commands::File { file_path } => file(file_path)?,
            Commands::Getconf => getconf(),
            Commands::Ls => ls()?,
            Commands::Mkdir { names } => mkdir(names)?,
            Commands::Rm { files } => rm(files)?,
            Commands::Rmdir { dirs } => rmdir(dirs)?,
            Commands::Sleep { duration_in_secs } => sleep(duration_in_secs),
            Commands::Wc {
                files,
                newlines,
                words,
                bytes,
            } => wc(files, newlines, words, bytes)?,
        };
        if let Some(content) = output {
            println!("{}\n{}", content, "Execution successful".bold());
        } else {
            println!("\nExecution successful, no output available");
        }
    } else {
        println!("Maybe next time...");
    }
    Ok(())
}
