use std::collections::HashMap;

use aoc_common_rust::{errors::AocError, input::InputReader};

pub(crate) fn parse_input(
    filename: &str,
) -> Result<(Vec<char>, HashMap<String, (String, String)>), AocError> {
    let lines = InputReader::as_paragraphs(&filename)?;

    let instructions: Vec<char> = lines[0].chars().collect();

    let graph_parts: Vec<(String, (String, String))> = lines[1]
        .split('\n')
        .map(|line| {
            let (elements, left_right): (&str, &str) = line.split_once(" = ").unwrap();

            let binding = left_right.replace('(', "").replace(')', "");
            let (left, right): (&str, &str) = binding.split_once(", ").unwrap();

            return (elements.into(), (left.into(), right.into()));
        })
        .collect();
    let graph: HashMap<String, (String, String)> = graph_parts
        .into_iter()
        .map(|(key, value)| (key, value))
        .collect();

    Ok((instructions, graph))
}

pub(crate) fn get_n_steps(
    instructions: Vec<char>,
    graph: HashMap<String, (String, String)>,
    from: &str,
    to: &str,
) -> u32 {
    let mut n_steps = 0;
    let mut index = 0;
    let n_instructions = instructions.len();

    let mut curr_node = from;
    loop {
        println!("steps: {n_steps}");
        println!("index: {index}/{n_instructions}");
        println!("curr: {curr_node}");

        if curr_node == to {
            break;
        }

        if index == n_instructions {
            index = 0;
        }

        let next_nodes = graph.get(curr_node).unwrap();
        let next_instruction = instructions[index];
        println!("next_nodes: {next_nodes:?}");

        curr_node = if next_instruction == 'L' {
            println!("moving {} to {}", next_instruction, next_nodes.0);
            &next_nodes.0
        } else {
            println!("moving {} to {}", next_instruction, next_nodes.1);
            &next_nodes.1
        };
        println!("now at {curr_node}");

        index += 1;
        n_steps += 1;
        println!("")
    }

    return n_steps;
}

fn get_n_steps_to_z(
    instructions: &Vec<char>,
    graph: &HashMap<String, (String, String)>,
    from: &str,
) -> u64 {
    let mut n_steps = 0;
    let mut index = 0;
    let n_instructions = instructions.len();

    let mut curr_node = from;
    loop {
        if curr_node.ends_with('Z') {
            break;
        }

        if index == n_instructions {
            index = 0;
        }

        let next_nodes = graph.get(curr_node).unwrap();
        let next_instruction = instructions[index];

        curr_node = if next_instruction == 'L' {
            &next_nodes.0
        } else {
            &next_nodes.1
        };

        index += 1;
        n_steps += 1;
    }

    return n_steps;
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    numbers.iter().copied().reduce(lcm).unwrap()
}

pub(crate) fn get_n_steps_2_spooky_boogaloo(
    instructions: Vec<char>,
    graph: HashMap<String, (String, String)>,
) -> u64 {
    let mut curr_nodes: Vec<&String> = graph.keys().filter(|k| k.ends_with('A')).collect();
    let cycle_lengths: Vec<u64> = curr_nodes
        .iter()
        .map(|start| get_n_steps_to_z(&instructions, &graph, start))
        .collect();

    return lcm_of_vec(&cycle_lengths);
}
