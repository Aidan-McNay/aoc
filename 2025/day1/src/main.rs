use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let src_file = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let file = File::open(src_file).unwrap();
    let reader = BufReader::new(file);
    let mut curr_num = 50;
    let mut pointed_at_zero = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let (direction, amount) = line
            .char_indices()
            .nth(1)
            .map(|(idx, _)| line.split_at(idx))
            .unwrap();
        let amount_num: i32 = amount.parse().unwrap();
        match direction {
            "L" => {
                curr_num -= amount_num;
            }
            "R" => {
                curr_num += amount_num;
            }
            _ => (),
        };
        while curr_num < 0 {
            curr_num += 100;
        }
        while curr_num >= 100 {
            curr_num -= 100
        }
        if curr_num == 0 {
            pointed_at_zero += 1;
        }
    }
    println! {"[Part1] Ended at zero {} times!", pointed_at_zero}
}

fn part2() {
    let src_file = std::env::args().nth(1).expect("no source file given");
    let file = File::open(src_file).unwrap();
    let reader = BufReader::new(file);
    let mut curr_num = 50;
    let mut prev_num: i32;
    let mut clicked_at_zero = 0;
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let (direction, amount) = line
            .char_indices()
            .nth(1)
            .map(|(idx, _)| line.split_at(idx))
            .unwrap();
        let amount_num: i32 = amount.parse().unwrap();
        prev_num = curr_num;
        match direction {
            "L" => {
                curr_num -= amount_num;
            }
            "R" => {
                curr_num += amount_num;
            }
            _ => (),
        };
        while curr_num < 0 {
            // Don't count as clicking over if we started there
            if prev_num != 0 {
                clicked_at_zero += 1;
            } else {
                // Count subsequent clicks over
                prev_num = 1;
            }
            curr_num += 100;
        }
        while curr_num >= 100 {
            // Exactly as 100 should count as ending there, not clicking over
            if curr_num != 100 {
                clicked_at_zero += 1;
            }
            curr_num -= 100;
        }
        if curr_num == 0 {
            clicked_at_zero += 1;
        }
    }
    println! {"[Part2] Clicked at zero {} times!", clicked_at_zero}
}

fn main() {
    part1();
    part2();
}
