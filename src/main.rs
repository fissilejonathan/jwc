use clap::Parser;
use file_stat::FileStat;
use prettytable::{row, Cell, Row, Table};
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::Result;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};

pub mod file_stat;

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

fn process_input_file(input_file: &String) -> Result<Vec<String>> {
    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);

    let mut input_files = Vec::<String>::new();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            input_files.push(line);
        }
    }

    return Ok(input_files);
}

fn setup_table(bytes: bool, chars: bool, lines: bool, max_line_length: bool, words: bool) -> Table {
    let mut table = Table::new();

    // default outputf
    if !bytes && !chars && !lines {
        table.add_row(row!["FILE", "BYTES", "CHARS", "LINES"]);
    } else {
        let mut cells = Vec::<Cell>::new();

        cells.push(Cell::new("FILE"));

        if bytes {
            cells.push(Cell::new("BYTES"));
        }

        if chars {
            cells.push(Cell::new("CHARS"));
        }

        if lines {
            cells.push(Cell::new("LINES"));
        }

        if max_line_length {
            cells.push(Cell::new("MAX LINE LENGTH"));
        }

        if words {
            cells.push(Cell::new("WORDS"));
        }

        table.add_row(Row::new(cells));
    }

    table
}

fn main() {
    let args = Args::parse();

    let table: Arc<Mutex<Table>> = Arc::new(Mutex::new(setup_table(
        args.bytes,
        args.chars,
        args.lines,
        args.max_line_length,
        args.words,
    )));

    if let Some(read_from) = args.read_from {
        if read_from == "-" {
            let dash_index = env::args()
                .into_iter()
                .position(|arg| arg.ends_with("-"))
                .unwrap();

            let input_files: Vec<String> = env::args().skip(dash_index + 1).collect();

            let rows = input_files
                .par_iter()
                .filter_map(|input_file| process_input_file(input_file).ok())
                .flat_map(|file_paths| file_paths)
                .map(|file_path| {});
        } else {
            let input_files: Vec<String> = read_from.split(",").map(|s| s.to_string()).collect();

            println!("{:?}", input_files);
        }
    } else {
        if let Some(files) = args.files {
            let rows = files.par_iter().map(|file_path| {
                let file_stat = FileStat::try_from(file_path)?;

                let mut cells = Vec::<Cell>::new();

                cells.push(Cell::new(&file_stat.file_name));

                if !args.bytes && !args.chars && !args.lines {
                    cells.push(Cell::new(&file_stat.byte_count.to_string()));
                    cells.push(Cell::new(&file_stat.char_count.to_string()));
                    cells.push(Cell::new(&file_stat.line_count.to_string()));
                } else {
                    if args.bytes {
                        cells.push(Cell::new(&file_stat.byte_count.to_string()));
                    }

                    if args.chars {
                        cells.push(Cell::new(&file_stat.char_count.to_string()));
                    }

                    if args.lines {
                        cells.push(Cell::new(&file_stat.line_count.to_string()));
                    }

                    if args.max_line_length {
                        cells.push(Cell::new(&file_stat.max_line_length.to_string()));
                    }

                    if args.words {
                        cells.push(Cell::new(&file_stat.word_count.to_string()));
                    }
                }

                Ok(Row::new(cells))
            });

            rows.for_each(|row_result: Result<Row>| {
                match row_result {
                    Ok(row) => {
                        // Lock the mutex to access the table
                        let mut locked_table = table.lock().unwrap();

                        locked_table.add_row(row);
                    }
                    Err(e) => println!("{}", e),
                }
            });
        }
    }

    let locked_table = table.lock().unwrap();
    locked_table.printstd();
}
