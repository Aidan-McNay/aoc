use utils::FileReader;

struct Dial {
    loc: i32,
    pub num_zero_stops: i32,
    pub num_zero_clicks: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            loc: 50,
            num_zero_stops: 0,
            num_zero_clicks: 0,
        }
    }
}

enum Rotation {
    Left(i32),
    Right(i32),
}

impl Dial {
    pub fn turn(&mut self, rotation: Rotation) {
        let prev_loc = self.loc;
        match rotation {
            Rotation::Left(amt) => {
                self.loc -= amt;
            }
            Rotation::Right(amt) => {
                self.loc += amt;
            }
        }
        match self.loc {
            _ if self.loc < 0 => {
                if prev_loc == 0 {
                    self.num_zero_clicks -= 1;
                };
                loop {
                    self.loc += 100;
                    self.num_zero_clicks += 1;
                    if self.loc >= 0 {
                        break;
                    }
                }
            }
            _ if self.loc >= 100 => {
                loop {
                    self.loc -= 100;
                    self.num_zero_clicks += 1;
                    if self.loc < 100 {
                        break;
                    }
                }
                if self.loc == 0 {
                    self.num_zero_clicks -= 1;
                }
            }
            _ => (),
        }
        if self.loc == 0 {
            self.num_zero_stops += 1;
            self.num_zero_clicks += 1;
        }
    }
}

fn parse(line: &str) -> Option<Rotation> {
    match &line[0..1] {
        "L" => Some(Rotation::Left(line[1..].parse().unwrap())),
        "R" => Some(Rotation::Right(line[1..].parse().unwrap())),
        _ => None,
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let finished_dial = FileReader::new(file_name.as_str())
        .filter_map(|line| parse(line.as_str()))
        .fold(Dial::default(), |mut dial, rot| {
            dial.turn(rot);
            dial
        });
    println! {"[Part1] Ended at zero {} times!", finished_dial.num_zero_stops}
    println! {"[Part2] Clicked at zero {} times!", finished_dial.num_zero_clicks}
}
