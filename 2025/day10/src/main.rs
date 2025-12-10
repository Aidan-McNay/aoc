use good_lp::{Expression, ProblemVariables, Solution, SolverModel, Variable, microlp, variable};
use std::collections::HashSet;
use utils::FileReader;

#[derive(Debug)]
struct Machine {
    goal: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_reqs: Vec<usize>,
}

impl From<String> for Machine {
    fn from(value: String) -> Self {
        let mut entries = value.split(" ").map(|e| e.to_string());
        let goal = entries
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                '[' | ']' => None,
                c => {
                    panic!("{} found in machine goal", c);
                }
            })
            .collect::<Vec<bool>>();
        let mut remaining_entries: Vec<Vec<usize>> = entries
            .map(|s| {
                s.replace("(", "")
                    .replace(")", "")
                    .replace("{", "")
                    .replace("}", "")
                    .split(",")
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();
        let joltage_reqs = remaining_entries.pop().unwrap();
        Self {
            goal,
            buttons: remaining_entries,
            joltage_reqs: joltage_reqs,
        }
    }
}

fn press_goal_button(mut curr_state: Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    for button_num in button {
        curr_state[*button_num] = !curr_state[*button_num]
    }
    curr_state
}

impl Machine {
    pub fn num_goal_presses(&self) -> usize {
        let mut states_to_check: Vec<Vec<bool>> = vec![vec![false; self.goal.len()]];
        let mut prev_states: HashSet<Vec<bool>> = HashSet::from([vec![false; self.goal.len()]]);
        let mut loop_iters = 0;
        loop {
            let mut next_states_to_check: Vec<Vec<bool>> = vec![];
            for state_to_check in states_to_check {
                for button in self.buttons.iter() {
                    let new_state = press_goal_button(state_to_check.clone(), button);
                    if new_state == self.goal {
                        return 1 + loop_iters;
                    }
                    if prev_states.contains(&new_state) {
                        continue;
                    }
                    next_states_to_check.push(new_state.clone());
                    prev_states.insert(new_state);
                }
            }
            states_to_check = next_states_to_check;
            loop_iters += 1;
        }
    }
    pub fn num_joltage_presses(&self) -> usize {
        let mut problem = ProblemVariables::new();
        let variables: Vec<Variable> =
            problem.add_all(vec![variable().integer().min(0); self.buttons.len()]);
        let objective: Expression = variables.iter().sum();
        let mut model = problem.minimise(objective).using(microlp);
        for (idx, sum) in self.joltage_reqs.iter().enumerate() {
            let mut sum_expr: Expression = Expression::from(0);
            for (button_idx, button) in self.buttons.iter().enumerate() {
                if button.contains(&idx) {
                    sum_expr = variables[button_idx] + sum_expr;
                }
            }
            model = model.with(sum_expr.eq(Expression::from(*sum as i32)));
        }
        let solution = model.solve().unwrap();
        variables
            .into_iter()
            .map(|var| solution.value(var).round() as i32)
            .fold(0, |acc, element| acc + element) as usize
    }
}

fn main() {
    let file_name = std::env::args().nth(1).expect("Usage: <binary> input.txt");
    let machines: Vec<Machine> = FileReader::new(file_name.as_str())
        .into_iter()
        .map(|line| Machine::from(line))
        .collect();
    let num_goal_presses = machines
        .iter()
        .fold(0, |acc, element| acc + element.num_goal_presses());
    println!(
        "[Part1] Toggling all indicator lights took {} presses",
        num_goal_presses
    );
    let num_joltage_presses = machines
        .iter()
        .fold(0, |acc, element| acc + element.num_joltage_presses());
    println!(
        "[Part2] Toggling all joltages took {} presses",
        num_joltage_presses
    );
}
