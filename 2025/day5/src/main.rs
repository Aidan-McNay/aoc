use std::cmp::{max, min};
use std::ops::RangeInclusive;
use utils::FileReader;

type IngredientID = u64;

fn num_elements(range: RangeInclusive<IngredientID>) -> u64 {
    let tuple_range = range.into_inner();
    (tuple_range.1 - tuple_range.0) + 1
}

struct FreshRanges {
    ranges: Vec<RangeInclusive<IngredientID>>,
}

impl FreshRanges {
    fn new(ranges: Vec<RangeInclusive<IngredientID>>) -> Self {
        Self { ranges: ranges }
    }
    fn is_fresh(&self, id: IngredientID) -> bool {
        self.ranges.iter().any(|range| range.contains(&id))
    }
    fn get_total_fresh(&mut self) -> u64 {
        let mut gaps: Vec<RangeInclusive<IngredientID>> = vec![];
        self.ranges.sort_by(|r1, r2| {
            if r1.start() != r2.start() {
                r1.start().cmp(r2.start())
            } else {
                r1.end().cmp(r2.end())
            }
        });
        let first_range = self.ranges.first().unwrap().clone();
        let total_range = self
            .ranges
            .iter()
            .fold(first_range, |prev_range, next_range| {
                assert!(next_range.end() >= next_range.start());
                let prev_range_tuple = prev_range.into_inner();
                let next_range_tuple = next_range.clone().into_inner();
                if prev_range_tuple.1 < (next_range_tuple.0 - 1) {
                    gaps.push(RangeInclusive::new(
                        prev_range_tuple.1 + 1,
                        next_range_tuple.0 - 1,
                    ));
                };
                RangeInclusive::new(
                    min(prev_range_tuple.0, next_range_tuple.0),
                    max(prev_range_tuple.1, next_range_tuple.1),
                )
            });
        num_elements(total_range)
            - gaps
                .into_iter()
                .fold(0, |acc, element| acc + num_elements(element))
    }
}

fn parse_database(mut str_iter: impl Iterator<Item = String>) -> (FreshRanges, Vec<IngredientID>) {
    let ranges = str_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|range| {
            let range_comps = range.split("-").collect::<Vec<&str>>();
            RangeInclusive::new(
                range_comps[0].parse().unwrap(),
                range_comps[1].parse().unwrap(),
            )
        })
        .collect::<Vec<RangeInclusive<u64>>>();
    let fresh_ranges = FreshRanges::new(ranges);
    let ingredient_ids = str_iter
        .map(|line| line.parse().unwrap())
        .collect::<Vec<IngredientID>>();
    (fresh_ranges, ingredient_ids)
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let (mut fresh_ranges, ingredient_ids) = parse_database(FileReader::new(file_name.as_str()));
    let num_fresh = ingredient_ids
        .into_iter()
        .filter(|id| fresh_ranges.is_fresh(*id))
        .count();
    println!("[Part1] {} ingredients are fresh", num_fresh);
    println!(
        "[Part2] {} possible fresh ingredients",
        fresh_ranges.get_total_fresh()
    );
}
