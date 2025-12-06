use std::str::FromStr;
use utils::FileReader;

#[derive(Debug)]
enum Op {
    Multiply,
    Add,
}

impl FromStr for Op {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Op::Multiply),
            "+" => Ok(Op::Add),
            _ => Err(format!("Invalid operation: {}", s)),
        }
    }
}

impl Op {
    fn init_val(&self) -> u64 {
        match *self {
            Op::Multiply => 1,
            Op::Add => 0,
        }
    }
    fn apply(&self, arg1: u64, arg2: u64) -> u64 {
        match *self {
            Op::Multiply => arg1 * arg2,
            Op::Add => arg1 + arg2,
        }
    }
}

fn grand_total(str_iter: impl Iterator<Item = String>) -> u64 {
    let mut str_vecs: Vec<Vec<String>> = str_iter
        .map(|line| line.split_whitespace().map(|str| str.to_string()).collect())
        .collect();
    let op_vec: Vec<Op> = str_vecs
        .pop()
        .unwrap()
        .into_iter()
        .map(|str| str.parse().unwrap())
        .collect();
    let arg_vecs: Vec<Vec<u64>> = str_vecs
        .into_iter()
        .map(|arg_vec| {
            arg_vec
                .into_iter()
                .map(|str| str.parse().unwrap())
                .collect()
        })
        .collect();
    let init_vals = op_vec.iter().map(|op| op.init_val()).collect::<Vec<u64>>();
    let result_vec = arg_vecs.iter().fold(init_vals, |curr_vals, new_args| {
        curr_vals
            .into_iter()
            .enumerate()
            .map(|(idx, element)| op_vec[idx].apply(element, new_args[idx]))
            .collect()
    });
    result_vec.into_iter().fold(0, |acc, element| acc + element)
}

fn grand_cephalopod_total(str_iter: impl Iterator<Item = String>) -> u64 {
    let mut str_vecs: Vec<String> = str_iter.collect();
    let num_cols = str_vecs[0].len();
    let op_vec: Vec<Op> = str_vecs
        .pop()
        .unwrap()
        .split_whitespace()
        .into_iter()
        .map(|str| str.parse().unwrap())
        .collect();
    let column_strs = str_vecs
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .fold(vec!["".to_string(); num_cols], |acc_vec, new_char_vec| {
            acc_vec
                .into_iter()
                .zip(new_char_vec)
                .map(|(mut curr_str, new_char)| {
                    curr_str.push(new_char);
                    curr_str
                })
                .collect()
        });
    let mut arg_vecs: Vec<Vec<u64>> = vec![];
    let last_arg_vec = column_strs
        .into_iter()
        .fold(vec![], |mut curr_args, new_arg| match new_arg.trim() {
            "" => {
                arg_vecs.push(curr_args);
                vec![]
            }
            new_arg => {
                curr_args.push(new_arg.parse().unwrap());
                curr_args
            }
        });
    if !last_arg_vec.is_empty() {
        arg_vecs.push(last_arg_vec);
    }
    arg_vecs
        .into_iter()
        .enumerate()
        .map(|(idx, arg_vec)| {
            let op = &op_vec[idx];
            arg_vec
                .into_iter()
                .fold(op.init_val(), |acc, new_element| op.apply(acc, new_element))
        })
        .fold(0, |acc, new_element| acc + new_element)
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    println!(
        "[Part1] The grand total is {:?}",
        grand_total(FileReader::new(file_name.as_str()))
    );
    println!(
        "[Part2] The grand total is {:?}",
        grand_cephalopod_total(FileReader::new(file_name.as_str()))
    );
}
