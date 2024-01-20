use std::{
    fs::File,
    io::{self, BufRead},
};

use std::convert::TryFrom;

pub struct FileStat {
    pub file_name: String,
    pub byte_count: usize,
    pub char_count: usize,
    pub line_count: usize,
    pub max_line_length: usize,
    pub word_count: usize,
}

impl TryFrom<&String> for FileStat {
    type Error = std::io::Error;

    fn try_from(file_path: &String) -> Result<Self, Self::Error> {
        let file = File::open(&file_path)?;
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
            max_line_length,
        })
    }
}
