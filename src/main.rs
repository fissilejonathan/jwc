use clap::Parser;

#[derive(Parser)]
#[command(name = "jwc")]
#[command(author = "Jonathan Morales")]
#[command(version = "1.0")]
#[command(about = "Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified. With no FILE, or when FILE is -, r", long_about = None)]
struct Args {
    /// Print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Print the character counts
    #[arg(short = 'm', long)]
    chars: bool,

    /// Print the newline counts
    #[arg(short, long)]
    lines: bool,

    /// Read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input else input should be comma separated
    #[arg(id = "read-from", long, value_name = "F")]
    read_from: Option<String>,

    /// Print the length of the longest line
    #[arg(short = 'L', long)]
    max_line_length: bool,

    /// Print the word counts
    #[arg(short, long)]
    words: bool,

    /// Files that will be processed; Can be one or more
    files: Option<Vec<String>>,
}

fn main() {
    let cli = Args::parse();

    println!("read_from: {:?}", cli.read_from);
    println!("files: {:?}", cli.files);
}
