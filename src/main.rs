use clap::Parser;

#[derive(Parser)]
#[command(name = "jwc")]
#[command(author = "Jonathan Morales")]
#[command(version = "1.0")]
#[command(about = "Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified. With no FILE, or when FILE is -, r", long_about = None)]
struct Args {
    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// print the character counts
    #[arg(short = 'm', long)]
    chars: bool,

    /// print the newline counts
    #[arg(short, long)]
    lines: bool,

    /// read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input else input should be comma separated
    #[arg(id = "read-from", long, value_name = "F")]
    read_from: Option<String>,

    /// print the length of the longest line
    #[arg(short = 'L', long)]
    max_line_length: bool,

    /// print the word counts
    #[arg(short, long)]
    words: bool,

    /// files that will be processed; can be one or more
    files: Option<Vec<String>>,
}

fn main() {
    let cli = Args::parse();

    println!("read_from: {:?}", cli.read_from);
    println!("files: {:?}", cli.files);
}
