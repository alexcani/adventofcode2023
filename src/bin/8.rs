use advent_of_code_2023::read_lines_as_vec;
use regex::Regex;
use std::collections::HashMap;

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

// Part 1
// Iterate over sequence until we find node ZZZ, count the number of steps
fn calculate_part_1(sequence: &str, nodes: &NodeMap, starting_node: &str) -> u32 {
    let mut steps = 0;
    let mut current_node = starting_node;

    if !nodes.contains_key(current_node) {
        return 0;
    }

    for char in sequence.chars().cycle() {
        steps += 1;
        current_node = if char == 'L' {
            nodes.get(current_node).unwrap().0
        } else {
            nodes.get(current_node).unwrap().1
        };

        if current_node.ends_with('Z') {
            break;
        }
    }

    steps
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

// Part 2 - Start simultaneously from all nodes that end with A, until they all simultaneously end in nodes ending with Z
// Count the number of steps
fn calculate_part_2(sequence: &str, nodes: &NodeMap) -> u64 {
    let starting_nodes = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    // Each node path is cyclic, calculate the cycles
    let cycles = starting_nodes.iter().map(|n| calculate_part_1(sequence, nodes, n) as u64).collect::<Vec<_>>();

    // Calculate least common multiple of all cycles, that's the number of steps
    let mut lcm = cycles[0];
    for &i in cycles.iter().skip(1) {
        lcm = lcm * i / gcd(lcm, i);
    }

    lcm
}

fn calculate_result(input: &[String]) -> (u32, u64) {
    let sequence = &input[0];
    let regex = Regex::new(r"((?:\d|\w)+) = \(((?:\d|\w)+), ((?:\d|\w)+)\)").unwrap();
    let mut nodes: NodeMap = HashMap::new();
    for line in input.iter().skip(2) {
        let captures = regex.captures(line).unwrap();
        let label = captures.get(1).unwrap().as_str();
        let left = captures.get(2).unwrap().as_str();
        let right = captures.get(3).unwrap().as_str();

        nodes.insert(label, (left, right));
    }

    println!("Running part 1");
    let part_1_steps = calculate_part_1(sequence, &nodes, "AAA");

    println!("Running part 2");
    let part_2_steps = calculate_part_2(sequence, &nodes);

    (part_1_steps, part_2_steps)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/8.txt").unwrap();

    let _example = [
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let _example_2 = [
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let _example_3 = [
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let (steps, steps_2) = calculate_result(&_lines);
    println!("Steps: {}", steps);
    println!("Steps 2: {}", steps_2);
}
