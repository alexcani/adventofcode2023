use advent_of_code_2023::read_lines_as_vec;
use regex::Regex;
use std::collections::HashMap;

fn calculate_result(input: &[String]) -> u32 {
    let sequence = &input[0];

    let regex = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in input.iter().skip(2) {
        let captures = regex.captures(line).unwrap();
        let label = captures.get(1).unwrap().as_str();
        let left = captures.get(2).unwrap().as_str();
        let right = captures.get(3).unwrap().as_str();

        nodes.insert(label, (left, right));
    }

    // Iterate over sequence until we find node ZZZ, count the number of steps
    let mut steps = 0;
    let mut current_node = "AAA";
    'outer: loop {
        for char in sequence.chars() {
            steps += 1;
            let target_node = if char == 'L' {
                nodes.get(current_node).unwrap().0
            } else {
                nodes.get(current_node).unwrap().1
            };

            if target_node == "ZZZ" {
                break 'outer;
            } else {
                current_node = target_node;
            }
        }
    }

    steps
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
    ].iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let _example_2 = [
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)"
    ].iter().map(|s| s.to_string()).collect::<Vec<String>>();

    let steps = calculate_result(&_lines);
    println!("Steps: {}", steps);
}
