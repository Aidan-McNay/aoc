use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_invalid_part1(id: i128) -> bool {
    let id_string = id.to_string();
    let id_string_len = id_string.len();
    match id_string_len % 2 {
        0 => {
            let half_len = id_string_len / 2;
            &id_string[0..half_len] == &id_string[half_len..id_string_len]
        }
        1 => false,
        _ => {
            panic!("Huh?!?!");
        }
    }
}

fn get_invalid_sum_for_range(start: i128, end: i128, invalid_checker: fn(i128) -> bool) -> i128 {
    let mut sum = 0;
    for value in start..=end {
        if invalid_checker(value) {
            sum += value;
        }
    }
    return sum;
}

fn part1() {
    let src_file = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let file = File::open(src_file).unwrap();
    let reader = BufReader::new(file);
    let mut overall_sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let ranges: Vec<&str> = line.split(",").collect();
        for range in ranges {
            let components: Vec<&str> = range.split("-").collect();
            let start = components[0]
                .parse::<i128>()
                .expect("Failed to parse string to i128");
            let end = components[1]
                .parse::<i128>()
                .expect("Failed to parse string to i128");
            overall_sum += get_invalid_sum_for_range(start, end, is_invalid_part1);
        }
    }
    println!("[Part1] Overall sum from all ranges is {}", overall_sum)
}

fn is_invalid_part2(id: i128) -> bool {
    let id_string = id.to_string();
    let id_string_len = id_string.len();
    for i in 1..=id_string_len / 2 {
        let substr = &id_string[0..i];
        let repetition_amount = id_string_len / i;
        if substr.repeat(repetition_amount) == id_string {
            return true;
        }
    }
    return false;
}

fn part2() {
    let src_file = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let file = File::open(src_file).unwrap();
    let reader = BufReader::new(file);
    let mut overall_sum = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let ranges: Vec<&str> = line.split(",").collect();
        for range in ranges {
            let components: Vec<&str> = range.split("-").collect();
            let start = components[0]
                .parse::<i128>()
                .expect("Failed to parse string to i128");
            let end = components[1]
                .parse::<i128>()
                .expect("Failed to parse string to i128");
            overall_sum += get_invalid_sum_for_range(start, end, is_invalid_part2);
        }
    }
    println!("[Part2] Overall sum from all ranges is {}", overall_sum)
}

fn main() {
    part1();
    part2();
}
