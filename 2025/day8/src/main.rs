use std::collections::HashMap;
use utils::FileReader;

// (x, y, z, id)
type JunctionBox = (usize, usize, usize, usize);
type Circuit = usize;

fn dist_squared(j1: &JunctionBox, j2: &JunctionBox) -> usize {
    j1.0.abs_diff(j2.0).pow(2) + j1.1.abs_diff(j2.1).pow(2) + j1.2.abs_diff(j2.2).pow(2)
}

fn connect_closest(
    boxes: &mut Vec<(Circuit, JunctionBox, Vec<usize>)>,
) -> (JunctionBox, JunctionBox) {
    let mut min_j1 = boxes.first().unwrap();
    let mut min_j2 = boxes.iter().find(|jbox| jbox.0 != min_j1.0).unwrap();
    let mut min_dist = dist_squared(&min_j1.1, &min_j2.1);
    for (idx, j1) in boxes.iter().enumerate() {
        for j2 in boxes.iter().skip(idx + 1) {
            if j1.2.contains(&j2.1.3) | j2.2.contains(&j1.1.3) {
                continue;
            }
            let box_dist = dist_squared(&j1.1, &j2.1);
            if box_dist < min_dist {
                min_dist = box_dist;
                min_j1 = j1;
                min_j2 = j2;
            }
        }
    }
    let circuit_to_join = min_j1.0;
    let circuit_to_join_from = min_j2.0;
    let id_to_connect = min_j2.1.3;
    let id_to_connect_to = min_j1.1.3;
    let result = (min_j1.1.clone(), min_j2.1.clone());
    for jbox in boxes.iter_mut() {
        if jbox.0 == circuit_to_join_from {
            jbox.0 = circuit_to_join;
        }
        if jbox.1.3 == id_to_connect_to {
            jbox.2.push(id_to_connect);
        }
    }
    result
}

fn parse_junction_box(coords: String, id: usize) -> JunctionBox {
    let mut components = coords.split(",");
    let x = components.next().unwrap().parse().unwrap();
    let y = components.next().unwrap().parse().unwrap();
    let z = components.next().unwrap().parse().unwrap();
    (x, y, z, id)
}

fn three_largest_product(boxes: &Vec<(Circuit, JunctionBox, Vec<usize>)>) -> usize {
    let mut sizes: HashMap<Circuit, usize> = HashMap::new();
    for (circuit, _, _) in boxes.iter() {
        sizes.entry(*circuit).and_modify(|s| *s += 1).or_insert(1);
    }
    let (mut largest, mut second_largest, mut third_largest) = (0, 0, 0);
    for (_, size) in sizes.iter() {
        if *size > third_largest {
            third_largest = *size;
        }
        if third_largest > second_largest {
            let tmp = second_largest;
            second_largest = third_largest;
            third_largest = tmp;
        }
        if second_largest > largest {
            let tmp = largest;
            largest = second_largest;
            second_largest = tmp;
        }
    }
    largest * second_largest * third_largest
}

fn more_than_one_circuit(boxes: &Vec<(Circuit, JunctionBox, Vec<usize>)>) -> bool {
    let mut box_iter = boxes.iter();
    let circuit = box_iter.next().unwrap().0;
    for (new_circuit, _, _) in box_iter {
        if *new_circuit != circuit {
            return true;
        }
    }
    return false;
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Usage: <binary> input.txt num_connections");
    let num_connections: usize = std::env::args()
        .nth(2)
        .expect("Usage: <binary> input.txt num_connections")
        .parse()
        .unwrap();
    let mut boxes: Vec<(Circuit, JunctionBox, Vec<usize>)> = FileReader::new(file_name.as_str())
        .into_iter()
        .enumerate()
        .map(|(idx, jbox_str)| (idx, parse_junction_box(jbox_str, idx), vec![idx]))
        .collect();
    let mut connected_boxes: (JunctionBox, JunctionBox) = ((0, 0, 0, 0), (0, 0, 0, 0));
    for _ in 0..num_connections {
        connected_boxes = connect_closest(&mut boxes);
    }
    println! {"[Part1] Product of largest circuits is {}", three_largest_product(&boxes)};
    while more_than_one_circuit(&boxes) {
        connected_boxes = connect_closest(&mut boxes);
    }
    let final_x_product = connected_boxes.0.0 * connected_boxes.1.0;
    println!(
        "[Part2] Product of final connected boxes' X-coord is {}",
        final_x_product
    );
}
