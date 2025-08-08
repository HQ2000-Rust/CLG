use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq, Eq, Clone)]
#[command()]
pub enum Commands {
    #[clap(
        about = "concatenate and display files",
        long_about = "concatenate and display one or more files with valid UTF-8 encoding"
    )]
    Cat { files: Vec<PathBuf> },
    #[clap(about = "copy a file", long_about = "copy a file")]
    Cp { file: PathBuf, new_file: PathBuf },
    #[clap(
        about = "only show a certain range for each line",
        long_about = "shows a certain range of characters per line based on an optional start and end index"
    )]
    Cut {
        file: PathBuf,
        #[arg(short, long, default_value = "1")]
        start: Option<u32>,
        #[arg(short, long)]
        end: Option<u32>,
    },
    #[clap(
        about = "display the local date and time",
        long_about = "display the local date (yyyy-MM-dd), time (hh:mm:ss.ns) and local UTC offset (hh:mm)"
    )]
    Date,
    #[clap(
        about = "display the name of the current directory",
        long_about = "display thge name of the current directory or \"..\" if its path terminates with \"..\""
    )]
    Dirname,
    #[clap(
        about = "output formatted text and access special variables",
        long_about = "outputs formatted text with \\n, \\t and \\r and access the variables \"!DIRNAME\" and \"!DATE\""
    )]
    Echo { name: String },
    #[clap(
        about = "display the MIME type of a file",
        long_about = "display the MIME type of the file at the specified path"
    )]
    File { file_path: PathBuf },
    #[clap(
        about = "get the configuration options",
        long_about = "get the configuration options, try it out yourself"
    )]
    Getconf,
    #[clap(
        about = "list all files and directories in the current directory",
        long_about = "list all files and directories in the current directory/scope (non-recursive)"
    )]
    Ls,
    #[clap(
        about = "create a directory",
        long_about = "create a directory with the specified name"
    )]
    Mkdir { names: Vec<String> },
    #[clap(
        about = "remove one or more files",
        long_about = "remove one or more file at the specified path(s)"
    )]
    Rm { files: Vec<PathBuf> },
    #[clap(
        about = "remove one or more directories",
        long_about = "remove one or more directory at the specified path(s)"
    )]
    Rmdir { dirs: Vec<PathBuf> },
    #[clap(
        about = "pause the execution of the program",
        long_about = "pause the execution of the program for the specified amount of time (in seconds)"
    )]
    Sleep { duration_in_secs: u32 },
    #[clap(
        about = "display word, byte and newline counts",
        long_about = "display word, byte and newline counts of one or more file at the specified path(s)\nflags do stack if multiple are defined"
    )]
    Wc {
        files: Vec<PathBuf>,
        #[arg(
            short = 'l',
            long,
            default_value = "false",
            help = "Wether only the newline count should be shown"
        )]
        newlines: bool,
        #[arg(
            short,
            long,
            default_value = "false",
            help = "Wether only the word count should be shown"
        )]
        words: bool,
        #[arg(
            short,
            long,
            default_value = "false",
            help = "Wether only the byte count should be shown"
        )]
        bytes: bool,
    },
}
