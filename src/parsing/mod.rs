use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
//#[clap(author, version, about, long_about = None)]
#[command(version, about, long_about = None)]
pub struct Cli {
    //later
    //
    #[arg(short, long, default_value = "1", value_name = "Number > 0 and =< 100", allow_negative_numbers = false)]
    pub difficulty: Option<u8>,
    //
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    //no alias, should be a standalone bin
    //bc
    //basename
    Cat {
        #[arg(short, long, value_name = "1+ file paths")]
        files: Vec<PathBuf>,
    },
    Cmp {
        #[arg(short, long, value_name = "file path")]
        file1: PathBuf,
        #[arg(short, long, value_name = "file path")]
        file2: PathBuf,
    },
    Comm {
        #[arg(short, long, value_name = "file path")]
        file1: PathBuf,
        #[arg(short, long, value_name = "file path")]
        file2: PathBuf,
    },
    Cp {
        #[arg(short, long, value_name = "file path")]
        file: PathBuf,
        #[arg(short, long, value_name = "new file path")]
        new_file: PathBuf,
    },
    Cut {
        #[arg(short, long, value_name = "file path")]
        file: PathBuf,
        //to be verified later in the program
        #[arg(short, long, value_name = "range, e. g. 11-80")]
        range: String,
    },
    Date,
    //dd
    //no diff probably
    Dirname,
    //maybe du
    Echo {
        #[arg(short, long)]
        args: Vec<String>,
    },
    //likely no ed
    //env not necessary
    //expand is prob too platform-specific
    //expr + false also not necessary
    File {
        #[arg(short, long, value_name = "file path")]
        file: PathBuf,
    },
    Fold {
        #[arg(short, long, value_name = "file path")]
        file: PathBuf,
        #[arg(short, long="words per line", value_name = "positive number", allow_negative_numbers = false)]
        wpl: u32,
    },
    //"there are no configs"
    Getconf,
    //no grep, can't use any external tool
    //maybe id?
    Ls,
    Mkdir,
    Printf {
        #[arg(short, long)]
        args: Vec<String>,
    },
    Pwd,
    Rm {
        #[arg(short, long, value_name = "file path")]
        files: Vec<PathBuf>,
    },
    Rmdir {
        #[arg(short, long, value_name = "file path")]
        dirs: Vec<PathBuf>,
    },
    //no sh sadly
    Sleep {
        #[arg(short, long, value_name = "seconds", allow_negative_numbers = false)]
        duration_in_secs: u32,
    },
    //maybe time, idk
    //maybe uname
    Wc {
        #[arg(short, long, value_name="boolean", default_value = "false")]
        bytes: bool,
    }
}
