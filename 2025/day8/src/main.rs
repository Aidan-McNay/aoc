use std::collections::HashMap;
use utils::FileReader;

// (x, y, z, id)
type JunctionBox = (usize, usize, usize, usize);

fn dist_squared(j1: &JunctionBox, j2: &JunctionBox) -> usize {
    j1.0.abs_diff(j2.0).pow(2) + j1.1.abs_diff(j2.1).pow(2) + j1.2.abs_diff(j2.2).pow(2)
}

fn get_sorted_connections(boxes: Vec<JunctionBox>) -> Vec<(usize, usize)> {
    let mut sorted_connections: Vec<(JunctionBox, JunctionBox)> = vec![];
    sorted_connections.reserve((boxes.len() * boxes.len() - 1) / 2);
    for (idx, j1) in boxes.iter().enumerate() {
        for j2 in boxes.iter().skip(idx + 1) {
            sorted_connections.push((j1.clone(), j2.clone()));
        }
    }
    sorted_connections.sort_by(|pair_1, pair_2| {
        dist_squared(&pair_1.0, &pair_1.1).cmp(&dist_squared(&pair_2.0, &pair_2.1))
    });
    let result = sorted_connections
        .into_iter()
        .map(|pair| (pair.0.3, pair.1.3))
        .collect();
    result
}

fn make_connection(
    connections: &mut impl Iterator<Item = (usize, usize)>,
    id_to_circuit_mappings: &mut HashMap<usize, usize>,
    circuit_sizes: &mut HashMap<usize, usize>,
) -> (usize, usize) {
    let (id1, id2) = connections.next().unwrap();
    let old_circuit = id_to_circuit_mappings.get(&id2).unwrap().clone();
    let new_circuit = id_to_circuit_mappings.get(&id1).unwrap().clone();
    let old_circuit_count = circuit_sizes.get(&old_circuit).unwrap().clone();
    if new_circuit != old_circuit {
        circuit_sizes
            .entry(new_circuit)
            .and_modify(|count| *count += old_circuit_count);
        circuit_sizes
            .entry(old_circuit)
            .and_modify(|count| *count = 0);
        *id_to_circuit_mappings = id_to_circuit_mappings
            .into_iter()
            .map(|(id, circuit)| {
                if *circuit == old_circuit {
                    (*id, new_circuit)
                } else {
                    (*id, *circuit)
                }
            })
            .collect()
    }
    (id1, id2)
}

fn parse_junction_box(coords: String, id: usize) -> JunctionBox {
    let mut components = coords.split(",");
    let x = components.next().unwrap().parse().unwrap();
    let y = components.next().unwrap().parse().unwrap();
    let z = components.next().unwrap().parse().unwrap();
    (x, y, z, id)
}

fn three_largest_product(circuit_sizes: HashMap<usize, usize>) -> usize {
    let mut sorted_sizes = circuit_sizes.into_iter().collect::<Vec<(usize, usize)>>();
    sorted_sizes.sort_by(|(_, size1), (_, size2)| size2.cmp(size1));
    sorted_sizes
        .into_iter()
        .take(3)
        .fold(1, |acc, element| acc * element.1)
}

fn more_than_one_circuit(circuit_sizes: &HashMap<usize, usize>) -> bool {
    let mut circuit_size = 0;
    for (_, size) in circuit_sizes.iter() {
        if *size == 0 {
            continue;
        }
        if circuit_size == 0 {
            circuit_size = *size;
        } else {
            return true;
        }
    }
    return false;
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Usage: <binary> input.txt <num_connections, default = 1000>");
    let num_connections: usize = std::env::args()
        .nth(2)
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap();
    let boxes: Vec<JunctionBox> = FileReader::new(file_name.as_str())
        .into_iter()
        .enumerate()
        .map(|(idx, jbox_str)| parse_junction_box(jbox_str, idx))
        .collect();
    let mut id_to_circuit_mappings: HashMap<usize, usize> =
        (0..boxes.len()).map(|id| (id, id)).collect();
    let mut circuit_sizes: HashMap<usize, usize> = (0..boxes.len()).map(|id| (id, 1)).collect();
    let mut connections = get_sorted_connections(boxes.clone()).into_iter();

    let mut connected_ids: (usize, usize) = (0, 0);
    for _ in 0..num_connections {
        connected_ids = make_connection(
            &mut connections,
            &mut id_to_circuit_mappings,
            &mut circuit_sizes,
        );
    }
    println! {"[Part1] Product of largest circuits is {}", three_largest_product(circuit_sizes.clone())};
    while more_than_one_circuit(&circuit_sizes) {
        connected_ids = make_connection(
            &mut connections,
            &mut id_to_circuit_mappings,
            &mut circuit_sizes,
        );
    }
    let final_x_product = boxes
        .iter()
        .find(|jbox| jbox.3 == connected_ids.0)
        .unwrap()
        .0
        * boxes
            .iter()
            .find(|jbox| jbox.3 == connected_ids.1)
            .unwrap()
            .0;
    println!(
        "[Part2] Product of final connected boxes' X-coord is {}",
        final_x_product
    );
}
