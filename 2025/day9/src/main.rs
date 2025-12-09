use itertools::Itertools;
use std::cmp::{max, min};
use utils::FileReader;

type Coordinate = (usize, usize);

fn area(coord1: Coordinate, coord2: Coordinate) -> usize {
    (coord1.0.abs_diff(coord2.0) + 1) * (coord1.1.abs_diff(coord2.1) + 1)
}

fn adjacent(coord1: Coordinate, coord2: Coordinate) -> bool {
    ((coord1.0.abs_diff(coord2.0) == 1) && (coord1.1 == coord2.1))
        || ((coord1.1.abs_diff(coord2.1) == 1) && (coord1.0 == coord2.0))
}

fn point_in_rect(rect: (Coordinate, Coordinate), point: Coordinate) -> bool {
    ((point.0 < rect.0.0 && point.0 > rect.1.0) || (point.0 > rect.0.0 && point.0 < rect.1.0))
        && ((point.1 < rect.0.1 && point.1 > rect.1.1)
            || (point.1 > rect.0.1 && point.1 < rect.1.1))
}

fn line_intersection(
    rect_line: (Coordinate, Coordinate),
    line: (Coordinate, Coordinate),
) -> Option<Coordinate> {
    // Only consider orthogonal lines
    if (line.0.0 != line.1.0) && (rect_line.0.1 != rect_line.1.1) {
        if (rect_line.0.0 > max(line.0.0, line.1.0)) || (rect_line.0.0 < min(line.0.0, line.1.0)) {
            return None;
        }
        if (line.0.1 > max(rect_line.0.1, rect_line.1.1))
            || (line.0.1 < min(rect_line.0.1, rect_line.1.1))
        {
            return None;
        }
        return Some((rect_line.0.0, line.0.1));
    }
    if (line.0.1 != line.1.1) && (rect_line.0.0 != rect_line.1.0) {
        if (rect_line.0.1 > max(line.0.1, line.1.1)) || (rect_line.0.1 < min(line.0.1, line.1.1)) {
            return None;
        }
        if (line.0.0 > max(rect_line.0.0, rect_line.1.0))
            || (line.0.0 < min(rect_line.0.0, rect_line.1.0))
        {
            return None;
        }
        return Some((line.0.0, rect_line.0.1));
    }
    None
}

fn crossover_point(rect: (Coordinate, Coordinate), line: (Coordinate, Coordinate)) -> Coordinate {
    let other_corners = ((rect.0.0, rect.1.1), (rect.1.0, rect.0.1));
    if let Some(coord) = line_intersection((rect.0, other_corners.0), line) {
        return coord;
    }
    if let Some(coord) = line_intersection((rect.0, other_corners.1), line) {
        return coord;
    }
    if let Some(coord) = line_intersection((rect.1, other_corners.0), line) {
        return coord;
    }
    if let Some(coord) = line_intersection((rect.1, other_corners.1), line) {
        return coord;
    }
    panic!("No intersection for {:?}, {:?}!", rect, line);
}

fn max_area_coords(coords: Vec<Coordinate>) -> Vec<(Coordinate, Coordinate, usize)> {
    let mut coord_combos = coords
        .iter()
        .combinations(2)
        .map(|coords| (*coords[0], *coords[1], area(*coords[0], *coords[1])))
        .collect::<Vec<(Coordinate, Coordinate, usize)>>();
    coord_combos.sort_by(|a, b| b.2.cmp(&a.2));
    coord_combos
}

fn lines(coords: Vec<Coordinate>) -> Vec<(Coordinate, Coordinate)> {
    let mut line_vec: Vec<(Coordinate, Coordinate)> = vec![];
    line_vec.reserve(coords.len());
    let last_coord = coords.last().unwrap().clone();
    coords
        .into_iter()
        .fold(last_coord, |prev_coord, new_coord| {
            line_vec.push((prev_coord, new_coord));
            new_coord
        });
    line_vec
}

fn enclosed(rect: (Coordinate, Coordinate), lines: &Vec<(Coordinate, Coordinate)>) -> bool {
    let mut last_exit: Option<Coordinate> = None;
    let mut last_entrance: Option<Coordinate> = None;
    for line in lines.iter() {
        let first_point_in_rect = point_in_rect(rect, line.0);
        let second_point_in_rect = point_in_rect(rect, line.1);
        let midpoint_in_rect =
            point_in_rect(rect, ((line.0.0 + line.1.0) / 2, (line.0.1 + line.1.1) / 2));
        if (midpoint_in_rect != first_point_in_rect) && (midpoint_in_rect != second_point_in_rect) {
            // Cuts across rectangle
            return false;
        }
        if first_point_in_rect && !second_point_in_rect {
            // Exiting
            let crossover = crossover_point(rect, *line);
            match (last_exit, last_entrance) {
                (None, None) => last_exit = Some(crossover),
                (_, Some(coord)) => {
                    if adjacent(coord, crossover) {
                        last_entrance = None
                    } else {
                        return false;
                    }
                }
                (Some(_), None) => {
                    return false;
                }
            }
        }
        if !first_point_in_rect && second_point_in_rect {
            // Entering
            let crossover = crossover_point(rect, *line);
            match (last_entrance, last_exit) {
                (None, None) => last_entrance = Some(crossover),
                (_, Some(coord)) => {
                    if adjacent(coord, crossover) {
                        last_exit = None
                    } else {
                        return false;
                    }
                }
                (Some(_), None) => {
                    return false;
                }
            }
        }
    }
    last_exit.is_none() && last_entrance.is_none()
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let coords = FileReader::new(file_name.as_str())
        .into_iter()
        .map(|line| {
            let mut components = line.split(",");
            (
                components.next().unwrap().parse().unwrap(),
                components.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<Coordinate>>();
    let sorted_coords = max_area_coords(coords.clone());
    println!("[Part1] Max area is {}", sorted_coords[0].2);
    let coord_lines = lines(coords);
    let max_enclosed_rect = sorted_coords
        .into_iter()
        .find(|(coord1, coord2, _)| enclosed((*coord1, *coord2), &coord_lines))
        .unwrap();
    println!("[Part2] Max area is {}", max_enclosed_rect.2);
}
