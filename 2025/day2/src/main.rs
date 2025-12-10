use std::ops::Range;
use std::vec::IntoIter;
use utils::FileReader;

struct RangeIterator {
    range_str_iter: IntoIter<String>,
}

impl RangeIterator {
    fn new(range_str: String) -> Self {
        Self {
            range_str_iter: range_str
                .split(",")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .into_iter(),
        }
    }
}

impl<'a> Iterator for RangeIterator {
    type Item = Range<u64>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.range_str_iter.next() {
            None => None,
            Some(range_str) => {
                let components: Vec<&str> = range_str.split("-").collect();
                let start = components[0]
                    .parse::<u64>()
                    .expect("Failed to parse string to i128");
                let end = components[1]
                    .parse::<u64>()
                    .expect("Failed to parse string to i128");
                Some(Range {
                    start: start,
                    end: end + 1,
                })
            }
        }
    }
}

trait MaybeInvalid {
    type InvalidConfig;
    fn is_invalid(&self, config: Self::InvalidConfig) -> bool;
}

impl MaybeInvalid for u64 {
    type InvalidConfig = fn(usize) -> Range<usize>;
    fn is_invalid(&self, config: Self::InvalidConfig) -> bool {
        let id_string = self.to_string();
        let id_string_len = id_string.len();
        for i in config(id_string_len) {
            let substr = &id_string[0..i];
            let repetition_amount = id_string_len / i;
            if substr.repeat(repetition_amount) == id_string {
                return true;
            }
        }
        return false;
    }
}

fn part1_invalid_repetition_range(id_str_len: usize) -> Range<usize> {
    Range {
        start: if id_str_len > 2 { id_str_len / 2 } else { 1 },
        end: (id_str_len / 2) + 1,
    }
}

fn part2_invalid_repetition_range(id_str_len: usize) -> Range<usize> {
    Range {
        start: 1,
        end: (id_str_len / 2) + 1,
    }
}

fn invalid_sum(range: Range<u64>, invalid_config: fn(usize) -> Range<usize>) -> u64 {
    range.fold(0, |acc, e| {
        if e.is_invalid(invalid_config) {
            acc + e
        } else {
            acc
        }
    })
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let (part1_sum, part2_sum) = FileReader::new(file_name.as_str())
        .flat_map(|line| RangeIterator::new(line))
        .map(|range| {
            (
                invalid_sum(range.clone(), part1_invalid_repetition_range),
                invalid_sum(range, part2_invalid_repetition_range),
            )
        })
        .fold((0, 0), |acc, element| {
            (acc.0 + element.0, acc.1 + element.1)
        });
    println!("[Part1] Overall sum from all ranges is {}", part1_sum);
    println!("[Part2] Overall sum from all ranges is {}", part2_sum);
}
