> **Setup instructions below!**
# CLG
No idea how original it is (I hope so), but basically your luck determines if and how the commands you type are executed.
Partially complies to the [POSIX standard](https://en.wikipedia.org/wiki/POSIX).

## Usage
> this is the same output as if you would run the --help flag alongside this CLI
  ### Commands
  - cat - concatenate and display files
  - cp - copy a file
  - cut - only show a certain range for each line
  - date -   display the local date and time
  - dirname - display the name of the current directory
  - echo - output formatted text and access special variables
  - file - display the MIME type of a file
  - getconf - get the configuration options
  - ls - list all files and directories in the current directory
  - mkdir - create a directory
  - rm - remove one or more files
  - rmdir - remove one or more directories
  - sleep - pause the execution of the program
  - wc - display word, byte and newline counts
  - help - Print this message or the help of the given subcommand(s)

### Options
-   -h, --help - Print help
-   -V, --version - Print version

## Setup
1. Download the latest release for your OS/arch like windows-x86-64
> If you don't find your OS and arch in the latest release, compile it (see **Compilation**
2. Put it into your desired directory with sufficient permissions (A "Playground" where you can safely delete or make new files and directories)
> You can optionally add it to the path, even though I would not recommend it because of its missing suitability
3. Run the executable `clgambling`
> 4. **Finished!**

## Compilation
1. Run `cargo install clgambling`
2. Run it as above stated

## Safety
- This is a CLI tool that contains commands like `rmdir` or `rm` which can unrecoverably delete potentially important/valueable files, depending on your OS (see the trash bin).
- Fortunately, you can only delete an empty directory or need to give the `rm` a complete list of the files to be deleted, so actually causing any accidental damage is mostly **very unlikely** 
- That mentioned, please only use this tool carefully, Control-C is always acceptable
