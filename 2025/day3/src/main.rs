use utils::FileReader;

trait MaxJoltage {
    fn get_max_joltage(&self, num_digits: u32) -> u64;
}

fn get_num(bank: &str, num_to_get: u32, min_idx: usize, max_idx: usize) -> Option<usize> {
    // Gets the index of the number at least min_idx into bank, at most max_idx, if present
    let char = std::char::from_digit(num_to_get, 10).unwrap();
    bank.chars()
        .enumerate()
        .position(|(idx, c)| (c == char) & (idx >= min_idx) & (idx <= max_idx))
}

fn get_max_joltage_helper(bank: &str, num_digits_required: u32, min_idx: usize) -> Option<u64> {
    if num_digits_required == 0 {
        return Some(0);
    }
    for i in (0..=9).rev() {
        let largest_num_idx = get_num(
            bank,
            i,
            min_idx,
            bank.len() - (num_digits_required as usize),
        );
        match largest_num_idx {
            None => continue,
            Some(idx) => match get_max_joltage_helper(bank, num_digits_required - 1, idx + 1) {
                None => continue,
                Some(lesser_num) => {
                    let coeff: u64 = 10_u64.pow(num_digits_required - 1);
                    return Some(((i as u64) * coeff) + lesser_num);
                }
            },
        }
    }
    None
}

impl MaxJoltage for &str {
    fn get_max_joltage(&self, num_digits: u32) -> u64 {
        get_max_joltage_helper(&self, num_digits, 0).unwrap()
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let (part1_joltage, part2_joltage) = FileReader::new(file_name.as_str())
        .map(|line| {
            let line_str = line.as_str();
            (line_str.get_max_joltage(2), line_str.get_max_joltage(12))
        })
        .fold((0, 0), |acc, element| {
            (acc.0 + element.0, acc.1 + element.1)
        });
    println!("[Part1] Max joltage of all banks is {}", part1_joltage);
    println!("[Part2] Max joltage of all banks is {}", part2_joltage);
}
