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

fn paths_to_out(devices: &Vec<Device>, curr_nodes: Vec<&String>) -> usize {
    if curr_nodes.is_empty() {
        return 0;
    }
    let mut new_nodes: Vec<&String> = vec![];
    let mut found_paths = 0;
    for node in curr_nodes {
        for device in devices {
            if device.name == *node {
                for next_device in &device.connected_to {
                    if next_device == "out" {
                        found_paths += 1;
                    } else {
                        new_nodes.push(&next_device)
                    }
                }
            }
        }
    }
    found_paths + paths_to_out(devices, new_nodes)
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let devices = FileReader::new(file_name.as_str())
        .map(|line| Device::from(line))
        .collect::<Vec<Device>>();
    let out_paths = paths_to_out(&devices, vec![&"you".to_owned()]);
    println!("[Part1] {} paths from you to out", out_paths);
}
