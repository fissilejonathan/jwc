use args::{Args, FlagArgs};
use clap::Parser;
use file_stat::FileStat;
use prettytable::{row, Cell, Row, Table};
use rayon::prelude::*;
use std::{
    env,
    fs::File,
    io::{self, BufRead, Result},
    sync::{Arc, Mutex},
};

pub mod args;
pub mod file_stat;

use std::convert::TryFrom;

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

fn process_file_stat(file_stat: &FileStat, flag_args: &FlagArgs) -> Row {
    let mut cells = Vec::<Cell>::new();

    cells.push(Cell::new(&file_stat.file_name));

    if !flag_args.bytes && !flag_args.chars && !flag_args.lines {
        cells.push(Cell::new(&file_stat.byte_count.to_string()));
        cells.push(Cell::new(&file_stat.char_count.to_string()));
        cells.push(Cell::new(&file_stat.line_count.to_string()));
    } else {
        if flag_args.bytes {
            cells.push(Cell::new(&file_stat.byte_count.to_string()));
        }

        if flag_args.chars {
            cells.push(Cell::new(&file_stat.char_count.to_string()));
        }

        if flag_args.lines {
            cells.push(Cell::new(&file_stat.line_count.to_string()));
        }

        if flag_args.max_line_length {
            cells.push(Cell::new(&file_stat.max_line_length.to_string()));
        }

        if flag_args.words {
            cells.push(Cell::new(&file_stat.word_count.to_string()));
        }
    }

    Row::new(cells)
}

fn create_table(flag_args: &FlagArgs) -> Table {
    let mut table = Table::new();

    // default output
    if !flag_args.bytes && !flag_args.chars && !flag_args.lines {
        table.add_row(row!["FILE", "BYTES", "CHARS", "LINES"]);
    } else {
        let mut cells = Vec::<Cell>::new();

        cells.push(Cell::new("FILE"));

        if flag_args.bytes {
            cells.push(Cell::new("BYTES"));
        }

        if flag_args.chars {
            cells.push(Cell::new("CHARS"));
        }

        if flag_args.lines {
            cells.push(Cell::new("LINES"));
        }

        if flag_args.max_line_length {
            cells.push(Cell::new("MAX LINE LENGTH"));
        }

        if flag_args.words {
            cells.push(Cell::new("WORDS"));
        }

        table.add_row(Row::new(cells));
    }

    table
}

fn main() {
    let args = Args::parse();
    let flag_args = FlagArgs::from(&args);

    let mut rows = Vec::<Result<Row>>::new();

    if let Some(read_from) = args.read_from {
        if read_from == "-" {
            let dash_index = env::args()
                .into_iter()
                .position(|arg| arg.ends_with("-"))
                .unwrap();

            let input_files: Vec<String> = env::args().skip(dash_index + 1).collect();

            rows = input_files
                .par_iter()
                .filter_map(|input_file| process_input_file(input_file).ok())
                .flat_map(|file_paths| file_paths)
                .map(|file_path| {
                    let file_stat = FileStat::try_from(&file_path)?;

                    Ok::<Row, std::io::Error>(process_file_stat(&file_stat, &flag_args))
                })
                .collect();
        } else {
            rows = read_from
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .par_iter()
                .map(|file_path| {
                    let file_stat = FileStat::try_from(file_path)?;

                    Ok::<Row, std::io::Error>(process_file_stat(&file_stat, &flag_args))
                })
                .collect();
        }
    } else {
        if let Some(files) = args.files {
            rows = files
                .par_iter()
                .map(|file_path| {
                    let file_stat = FileStat::try_from(file_path)?;

                    Ok::<Row, std::io::Error>(process_file_stat(&file_stat, &flag_args))
                })
                .collect();
        }
    }

    let table: Arc<Mutex<Table>> = Arc::new(Mutex::new(create_table(&flag_args)));

    rows.iter().for_each(|row_result: &Result<Row>| {
        match row_result {
            Ok(row) => {
                // Lock the mutex to access the table
                let mut locked_table = table.lock().unwrap();

                locked_table.add_row(row.clone());
            }
            Err(e) => println!("{}", e),
        }
    });

    let locked_table = table.lock().unwrap();
    locked_table.printstd();
}
