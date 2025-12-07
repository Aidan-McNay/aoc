use utils::FileReader;

struct Grid {
    chars: Vec<Vec<char>>,
    pub x: usize,
    pub y: usize,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            chars: vec![],
            x: 0,
            y: 0,
        }
    }
}

impl Grid {
    fn add_line(&mut self, line: String) {
        let new_char_vec: Vec<char> = line.chars().collect();
        if self.x == 0 {
            self.x = new_char_vec.len();
        }
        self.y += 1;
        self.chars.push(new_char_vec);
    }

    fn can_access(&self, x: i32, y: i32) -> bool {
        let coords_to_check = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .into_iter();
        let map = coords_to_check
            .map(|(curr_x, curr_y)| {
                if curr_x < 0 {
                    0
                } else if curr_y < 0 {
                    0
                } else if (curr_x, curr_y) == (x, y) {
                    0
                } else {
                    match self.chars.get(curr_y as usize) {
                        None => 0,
                        Some(inner_vec) => match inner_vec.get(curr_x as usize) {
                            None => 0,
                            Some(char) => (*char == '@') as i32,
                        },
                    }
                }
            })
            .fold(0, |acc, element| acc + element);
        map < 4
    }

    fn is_accessable(&self) -> Vec<Vec<bool>> {
        self.chars
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, c)| (*c == '@') & self.can_access(x as i32, y as i32))
                    .collect()
            })
            .collect()
    }

    fn num_accessable(&self) -> u64 {
        self.is_accessable()
            .into_iter()
            .flatten()
            .filter(|b| *b)
            .count() as u64
    }

    fn update_accessable(&mut self) -> u64 {
        let accessable_grid = self.is_accessable();
        let num_to_remove = accessable_grid.iter().flatten().filter(|b| **b).count() as u64;
        for (char_row, bool_row) in self.chars.iter_mut().zip(accessable_grid) {
            for (char, bool) in char_row.iter_mut().zip(bool_row) {
                if bool {
                    *char = 'X'
                }
            }
        }
        num_to_remove
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let mut grid = FileReader::new(file_name.as_str()).fold(Grid::default(), |mut grid, line| {
        grid.add_line(line);
        grid
    });
    println!("[Part1] {} rolls are accessible", grid.num_accessable());
    let mut total_removed = 0;
    loop {
        let more_removed = grid.update_accessable();
        total_removed += more_removed;
        if more_removed == 0 {
            break;
        }
    }
    println!("[Part2] {} rolls are accessible", total_removed);
}
