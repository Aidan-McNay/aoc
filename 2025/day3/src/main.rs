use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_num(bank: &str, num_to_get: u32, min_idx: usize, max_idx: usize) -> Option<usize> {
    // Gets the index of the number at least min_idx into bank, at most max_idx, if present
    let char = std::char::from_digit(num_to_get, 10).unwrap();
    bank.chars()
        .enumerate()
        .position(|(idx, c)| (c == char) & (idx >= min_idx) & (idx <= max_idx))
}

fn max_joltage_helper(bank: &str, num_digits_required: u32, min_idx: usize) -> Option<i128> {
    if num_digits_required == 0 {
        return Some(0);
    }
    for i in (0..=9).rev() {
        let largest_num = get_num(
            bank,
            i,
            min_idx,
            bank.len() - (num_digits_required as usize),
        );
        match largest_num {
            None => continue,
            Some(idx) => match max_joltage_helper(bank, num_digits_required - 1, idx + 1) {
                None => continue,
                Some(lesser_num) => {
                    let coeff: i128 = 10_i128.pow(num_digits_required - 1);
                    return Some(((i as i128) * coeff) + lesser_num);
                }
            },
        }
    }
    None
}

fn max_joltage_part1(bank: &str) -> i128 {
    match max_joltage_helper(bank, 2, 0) {
        None => panic!("Oops!"),
        Some(num) => num,
    }
}

fn max_joltage_part2(bank: &str) -> i128 {
    match max_joltage_helper(bank, 12, 0) {
        None => panic!("Oops!"),
        Some(num) => num,
    }
}

fn part1() {
    let src_file = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let file = File::open(src_file).unwrap();
    let reader = BufReader::new(file);
    let mut overall_sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        overall_sum += max_joltage_part1(&line);
    }
    println!("[Part1] Max joltage of all banks is {}", overall_sum);
}

fn part2() {
    let src_file = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let file = File::open(src_file).unwrap();
    let reader = BufReader::new(file);
    let mut overall_sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        overall_sum += max_joltage_part2(&line);
    }
    println!("[Part2] Max joltage of all banks is {}", overall_sum);
}

fn main() {
    part1();
    part2();
}
