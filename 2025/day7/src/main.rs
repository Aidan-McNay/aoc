use std::collections::HashMap;
use utils::FileReader;

fn progress(
    array: String,
    tachyons: HashMap<usize, usize>,
    split_count: &mut usize,
) -> HashMap<usize, usize> {
    let splitter_locs: Vec<usize> = array
        .chars()
        .enumerate()
        .filter_map(|(idx, c)| if c == '^' { Some(idx) } else { None })
        .collect();
    let mut new_tachyons: HashMap<usize, usize> = HashMap::new();
    for (loc, count) in tachyons {
        if splitter_locs.contains(&loc) {
            *split_count += 1;
            new_tachyons
                .entry(loc + 1)
                .and_modify(|prev_count| *prev_count += count)
                .or_insert(count);
            new_tachyons
                .entry(loc - 1)
                .and_modify(|prev_count| *prev_count += count)
                .or_insert(count);
        } else {
            new_tachyons
                .entry(loc)
                .and_modify(|prev_count| *prev_count += count)
                .or_insert(count);
        }
    }
    new_tachyons
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let mut lines = FileReader::new(file_name.as_str()).into_iter();
    // maps location -> count
    let tachyons: HashMap<usize, usize> = HashMap::from([(
        lines
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap(),
        1,
    )]);
    let mut split_count: usize = 0;
    let final_state = lines.fold(tachyons, |tachyons, line| {
        progress(line, tachyons, &mut split_count)
    });
    println!("[Part1] Tachyons split {} times", split_count);
    let timelines = final_state
        .into_iter()
        .fold(0, |acc, element| acc + element.1);
    println!("[Part2] {} timelines are possible", timelines);
}
