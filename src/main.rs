use clap::Parser;
use prettytable::{row, Cell, Row, Table};
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io::Result;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};

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

fn process_file(file_path: &String) -> Result<FileStat> {
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

    Ok(FileStat {
        file_name: file_path.to_string(),
        byte_count,
        char_count,
        line_count,
        word_count,
        max_line_length: max_line_length,
    })
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

    if let Some(read_from) = args.read_from {
        if read_from == "-" {
            let dash_index = env::args()
                .into_iter()
                .position(|arg| arg.ends_with("-"))
                .unwrap();

            let input_files: Vec<String> = env::args().skip(dash_index + 1).collect();

            println!("input_files {:?}", input_files);
        } else {
            let input_files: Vec<String> = read_from.split(",").map(|s| s.to_string()).collect();

            println!("{:?}", input_files);
        }
    } else {
        if let Some(files) = args.files {
            let rows = files.par_iter().map(|file_path| {
                let file_result = process_file(file_path);

                match file_result {
                    Ok(fs) => {
                        let mut cells = Vec::<Cell>::new();

                        cells.push(Cell::new(&fs.file_name));

                        if !args.bytes && !args.chars && !args.lines {
                            cells.push(Cell::new(&fs.byte_count.to_string()));
                            cells.push(Cell::new(&fs.char_count.to_string()));
                            cells.push(Cell::new(&fs.line_count.to_string()));
                        } else {
                            if args.bytes {
                                cells.push(Cell::new(&fs.byte_count.to_string()));
                            }

                            if args.chars {
                                cells.push(Cell::new(&fs.char_count.to_string()));
                            }

                            if args.lines {
                                cells.push(Cell::new(&fs.line_count.to_string()));
                            }

                            if args.max_line_length {
                                cells.push(Cell::new(&fs.max_line_length.to_string()));
                            }

                            if args.words {
                                cells.push(Cell::new(&fs.word_count.to_string()));
                            }
                        }

                        Ok(Row::new(cells))
                    }
                    Err(e) => Err(format!("{} - {}", file_path, e.kind())),
                }
            });

            let table: Arc<Mutex<Table>> = Arc::new(Mutex::new(setup_table(
                args.bytes,
                args.chars,
                args.lines,
                args.max_line_length,
                args.words,
            )));

            rows.for_each(|row_result| {
                match row_result {
                    Ok(row) => {
                        // Lock the mutex to access the table
                        let mut locked_table = table.lock().unwrap();

                        locked_table.add_row(row);
                    }
                    Err(e) => println!("{}", e),
                }
            });

            let locked_table = table.lock().unwrap();
            locked_table.printstd();
        }
    }
}
