use std::collections::HashMap;
use utils::FileReader;

struct Device {
    pub name: String,
    pub connected_to: Vec<String>,
}

impl From<String> for Device {
    fn from(value: String) -> Self {
        let mut components = value.split(" ").into_iter();
        let mut name = components.next().unwrap().to_owned();
        name.pop();
        let connected_to = components.map(|s| s.to_owned()).collect::<Vec<String>>();
        Self { name, connected_to }
    }
}

fn paths(
    devices: &Vec<Device>,
    curr_node: &String,
    target_node: &String,
    found_paths: &mut HashMap<(String, String), usize>,
) -> usize {
    if *curr_node == *target_node {
        return 1;
    }
    let Some(device) = devices.iter().find(|d| d.name == *curr_node) else {
        return 0;
    };
    device.connected_to.iter().fold(0, |acc, next_device| {
        match found_paths.get(&(next_device.clone(), target_node.clone())) {
            Some(num_paths) => acc + num_paths,
            None => {
                let num_paths = paths(devices, next_device, target_node, found_paths);
                found_paths.insert((next_device.clone(), target_node.clone()), num_paths);
                acc + num_paths
            }
        }
    })
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let devices = FileReader::new(file_name.as_str())
        .map(|line| Device::from(line))
        .collect::<Vec<Device>>();
    let mut found_paths: HashMap<(String, String), usize> = HashMap::new();
    let out_paths = paths(
        &devices,
        &"you".to_owned(),
        &"out".to_owned(),
        &mut found_paths,
    );
    println!("[Part1] {} paths from you to out", out_paths);
    let paths_to_find = vec![
        vec![
            "svr".to_owned(),
            "dac".to_owned(),
            "fft".to_owned(),
            "out".to_owned(),
        ],
        vec![
            "svr".to_owned(),
            "fft".to_owned(),
            "dac".to_owned(),
            "out".to_owned(),
        ],
    ];
    let total_paths = paths_to_find.into_iter().fold(0, |acc, path| {
        let mut num_paths = 1;
        let first_node = path.first().unwrap().clone();
        path.into_iter().fold(first_node, |prev_node, node| {
            num_paths *= paths(&devices, &prev_node, &node, &mut found_paths);
            node
        });
        acc + num_paths
    });
    println!(
        "[Part2] {} paths from svr to out through (dac, fft)",
        total_paths
    );
}
