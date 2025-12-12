use std::collections::HashSet;
use utils::FileReader;

type Coordinate = (usize, usize);

#[derive(Debug)]
enum Rotation {
    NONE,
    QUARTER,
    HALF,
    THREEQUARTERS,
}

#[derive(Debug)]
struct PresentShape {
    locs: Vec<Vec<bool>>,
}

fn modify_coord(coord: Coordinate, rotation: &Rotation, flipped: bool) -> Coordinate {
    let (mut i, mut j) = coord;
    if flipped {
        i = 2 - i;
        j = 2 - j;
    }
    match *rotation {
        Rotation::NONE => (),
        Rotation::QUARTER => {
            (i, j) = (2 - j, i);
        }
        Rotation::HALF => {
            (i, j) = (2 - i, 2 - j);
        }
        Rotation::THREEQUARTERS => {
            (i, j) = (j, 2 - i);
        }
    };
    (i, j)
}

impl PresentShape {
    fn from_lines(mut str_iter: impl Iterator<Item = String>) -> Self {
        str_iter.next();
        let locs = str_iter
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Self { locs }
    }

    fn size(&self) -> usize {
        self.locs
            .iter()
            .map(|bv| bv.iter().filter(|b| **b).count())
            .sum()
    }

    fn check_collide_and_place(
        &self,
        self_coord: &Coordinate,
        state: &mut Vec<Vec<bool>>,
        rotation: &Rotation,
        flipped: bool,
    ) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                let (mut abs_i, mut abs_j) = modify_coord((i, j), rotation, flipped);
                abs_i = abs_i + self_coord.0;
                abs_j = abs_j + self_coord.1;
                if self.locs[i][j] {
                    if state[abs_j][abs_i] {
                        return true;
                    } else {
                        state[abs_j][abs_i] = true;
                    }
                }
            }
        }
        false
    }
}

type PresentShapes = Vec<PresentShape>;

#[derive(Debug)]
struct Region {
    length: usize,
    height: usize,
    presents_required: Vec<usize>,
}

impl From<String> for Region {
    fn from(value: String) -> Self {
        let mut components = value.split(": ");
        let dimensions = components
            .next()
            .unwrap()
            .split("x")
            .map(|c| c.parse().unwrap())
            .collect::<Vec<usize>>();
        let presents_required = components
            .next()
            .unwrap()
            .split(" ")
            .map(|c| c.parse().unwrap())
            .collect::<Vec<usize>>();
        Self {
            length: dimensions[0],
            height: dimensions[1],
            presents_required,
        }
    }
}

fn can_fit_helper(
    state: Vec<Vec<bool>>,
    present_shapes: &PresentShapes,
    possible_placements: &Vec<(Coordinate, Rotation, bool)>,
    presents_to_fit: &Vec<usize>,
    curr_present_idx: usize,
    false_states: &mut HashSet<(Vec<Vec<bool>>, usize)>,
) -> bool {
    if curr_present_idx == presents_to_fit.len() {
        return true;
    }
    if false_states.contains(&(state.clone(), curr_present_idx)) {
        return false;
    }
    let present = &present_shapes[presents_to_fit[curr_present_idx]];
    for placement in possible_placements {
        let mut placement_state = state.clone();
        if !present.check_collide_and_place(
            &placement.0,
            &mut placement_state,
            &placement.1,
            placement.2,
        ) {
            if can_fit_helper(
                placement_state,
                present_shapes,
                possible_placements,
                presents_to_fit,
                curr_present_idx + 1,
                false_states,
            ) {
                return true;
            }
        }
    }
    false_states.insert((state, curr_present_idx));
    false
}

impl Region {
    fn possible_placements(&self) -> Vec<(Coordinate, Rotation, bool)> {
        let mut placements: Vec<(Coordinate, Rotation, bool)> = vec![];
        placements.reserve(8 * (self.length - 2) * (self.height - 2));
        for i in 0..=(self.length - 3) {
            for j in 0..=(self.height - 3) {
                placements.push(((i, j), Rotation::NONE, true));
                placements.push(((i, j), Rotation::NONE, false));
                placements.push(((i, j), Rotation::QUARTER, true));
                placements.push(((i, j), Rotation::QUARTER, false));
                placements.push(((i, j), Rotation::HALF, true));
                placements.push(((i, j), Rotation::HALF, false));
                placements.push(((i, j), Rotation::THREEQUARTERS, true));
                placements.push(((i, j), Rotation::THREEQUARTERS, false));
            }
        }
        placements
    }

    fn size(&self) -> usize {
        self.height * self.length
    }

    fn definitely_can_pack(&self) -> usize {
        (self.height / 3) * (self.length / 3)
    }

    fn presents_to_add(&self) -> Vec<usize> {
        let mut present_indeces: Vec<usize> = vec![];
        for (idx, count) in self.presents_required.iter().enumerate() {
            for _ in 0..*count {
                present_indeces.push(idx);
            }
        }
        present_indeces
    }

    fn start_state(&self) -> Vec<Vec<bool>> {
        vec![vec![false; self.length]; self.height]
    }

    fn can_fit(&self, present_shapes: &PresentShapes) -> bool {
        let presents_to_add = self.presents_to_add();
        if presents_to_add
            .iter()
            .map(|idx| present_shapes[*idx].size())
            .sum::<usize>()
            >= self.size()
        {
            return false;
        } else if presents_to_add.iter().count() <= self.definitely_can_pack() {
            return true;
        }
        // Never get here :)
        can_fit_helper(
            self.start_state(),
            present_shapes,
            &self.possible_placements(),
            &presents_to_add,
            0,
            &mut HashSet::new(),
        )
    }
}

type Regions = Vec<Region>;

fn parse_input(mut str_iter: impl Iterator<Item = String>) -> (PresentShapes, Regions) {
    let mut present_shapes: PresentShapes = vec![];
    loop {
        let next_lines = str_iter
            .by_ref()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<String>>();
        if next_lines.first().unwrap().ends_with(":") {
            present_shapes.push(PresentShape::from_lines(next_lines.into_iter()))
        } else {
            let regions = next_lines
                .into_iter()
                .map(|line| Region::from(line))
                .collect::<Regions>();
            break (present_shapes, regions);
        }
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let (present_shapes, regions) = parse_input(FileReader::new(file_name.as_str()));
    let valid_regions = regions
        .iter()
        .filter(|region| region.can_fit(&present_shapes))
        .count();
    println!("[Part1] {} regions where presents can fit", valid_regions);
}
