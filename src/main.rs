use clap::Parser;
use prettytable::{row, Cell, Row, Table};
use rayon::prelude::*;
use std::fs::File;
use std::io::Result;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "jwc")]
#[command(author = "Jonathan Morales")]
#[command(version = "1.0")]
#[command(about = "Print newline, word, and byte counts for each FILE, and a total line if more than one FILE is specified. With no FILE, or when FILE is -, r", long_about = None)]
struct Args {
    /// Print the byte counts
    #[arg(short, long)]
    bytes: bool,

    /// Print the character counts
    #[arg(short, long)]
    chars: bool,

    /// Print the newline counts
    #[arg(short, long)]
    lines: bool,

    /// Print the length of the longest line
    #[arg(short = 'L', long)]
    max_line_length: bool,

    /// Print the word counts
    #[arg(short, long)]
    words: bool,

    /// Read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input else input should be comma separated
    #[arg(id = "read-from", long, value_name = "F")]
    read_from: Option<String>,

    /// Files that will be processed; Can be one or more
    files: Option<Vec<String>>,
}

struct FileStat {
    file_name: String,
    byte_count: usize,
    char_count: usize,
    line_count: usize,
    max_line_length: usize,
    word_count: usize,
}

fn is_default_output(bytes_arg: bool, chars_arg: bool, lines_arg: bool) -> bool {
    !bytes_arg && !chars_arg && !lines_arg
}

fn process_file(file_path: &String) -> Result<Option<FileStat>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut byte_count = 0;
    let mut char_count: usize = 0;
    let mut line_count = 0;
    let mut word_count = 0;
    let mut max_line_length = 0;

    for line in reader.lines() {
        let line = line?;

        let line_byte_count: usize = line.len();

        byte_count += line_byte_count;
        char_count += line.chars().count();
        line_count += 1;
        word_count += line.split_whitespace().count();

        if line_byte_count > max_line_length {
            max_line_length = line_byte_count;
        }
    }

    Ok(Some(FileStat {
        file_name: file_path.to_string(),
        byte_count,
        char_count,
        line_count,
        word_count,
        max_line_length: max_line_length,
    }))
}

fn setup_table(args: &Args) -> Table {
    let mut table = Table::new();

    if is_default_output(args.bytes, args.chars, args.lines) {
        table.add_row(row!["FILE", "BYTES", "CHARS", "LINES"]);
    } else {
        let mut cells = Vec::<Cell>::new();

        cells.push(Cell::new("FILE"));

        if args.bytes {
            cells.push(Cell::new("BYTES"));
        }

        if args.chars {
            cells.push(Cell::new("CHARS"));
        }

        if args.lines {
            cells.push(Cell::new("LINES"));
        }

        if args.max_line_length {
            cells.push(Cell::new("MAX LINE LENGTH"));
        }

        if args.words {
            cells.push(Cell::new("WORDS"));
        }

        table.add_row(Row::new(cells));
    }

    table
}

fn main() {
    let args = Args::parse();

    if let Some(read_from) = args.read_from {
        println!("read_from: {:?}", read_from);

        if read_from == "-" {
            // read from standard input
        } else {
            // split input by commas
            // read from files
        }
    } else {
        if let Some(files) = args.files {
            let file_stats = files.par_iter().map(|file_path| process_file(file_path));

            // let table: Table = setup_table(args);

            // table.printstd();
        }
    }
}
