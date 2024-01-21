use clap::Parser;
use std::convert::From;

#[derive(Parser)]
#[command(name = "jwc")]
#[command(author = "Jonathan Morales")]
#[command(version = "1.0")]
#[command(about = "Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified. With no FILE, or when FILE is -, r", long_about = None)]
pub struct Args {
    /// Print the byte counts
    #[arg(short, long)]
    pub bytes: bool,

    /// Print the character counts
    #[arg(short, long)]
    pub chars: bool,

    /// Print the newline counts
    #[arg(short, long)]
    pub lines: bool,

    /// Print the length of the longest line
    #[arg(short = 'L', long)]
    pub max_line_length: bool,

    /// Print the word counts
    #[arg(short, long)]
    pub words: bool,

    /// Read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input else input should be comma separated
    #[arg(id = "read-from", long, value_name = "F")]
    pub read_from: Option<String>,

    /// Files that will be processed; Can be one or more
    pub files: Option<Vec<String>>,
}

pub struct FlagArgs {
    pub bytes: bool,
    pub chars: bool,
    pub lines: bool,
    pub max_line_length: bool,
    pub words: bool,
}

impl From<&Args> for FlagArgs {
    fn from(args: &Args) -> FlagArgs {
        FlagArgs {
            bytes: args.bytes,
            chars: args.chars,
            lines: args.lines,
            max_line_length: args.max_line_length,
            words: args.words,
        }
    }
}
