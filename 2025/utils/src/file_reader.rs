use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader};

pub struct FileReader {
    lines: Lines<BufReader<File>>,
}

impl FileReader {
    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        FileReader {
            lines: BufReader::new(file).lines(),
        }
    }
}

impl Iterator for FileReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.lines.next() {
                None => {
                    break None;
                }
                Some(line_result) => {
                    if let Ok(line) = line_result {
                        break Some(line);
                    }
                }
            }
        }
    }
}
