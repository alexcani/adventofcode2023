use std::collections::HashMap;

use advent_of_code_2023::read_lines_as_vec;
use regex::Regex;

// Rules take a part and return a string containing the next rule's label (or A/R in case of accept/reject),
// or None if the rule does not apply
type Rule = dyn Fn(&Part) -> Option<String>;
type Workflow = Vec<Box<Rule>>;
type Workflows = HashMap<String, Workflow>;  // label -> rules

fn approve(_: &Part) -> Option<String> {
    Some("A".to_string())
}

fn reject(_: &Part) -> Option<String> {
    Some("R".to_string())
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse(input: &[String]) -> (Vec<Part>, Workflows) {
    let mut parts = Vec::new();
    let mut workflows = HashMap::new();
    let part_re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let workflow_re = Regex::new(r"^(\w+)\{(.*)\}$").unwrap();

    for line in input {
        let line = line.trim();
        if line.is_empty() {
            continue;
        } else if line.starts_with('{') {
            // Parse part
            let captures = part_re.captures(line).unwrap();
            let x = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let m = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let a = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let s = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();
            parts.push(Part { x, m, a, s });
        } else {
            // Parse workflow
            let mut rules: Workflow = Vec::new();
            let captures = workflow_re.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();

            let rules_str = captures.get(2).unwrap().as_str();
            rules_str.split(',').for_each(|rule| {
                if rule == "A" {
                    rules.push(Box::new(approve));
                } else if rule == "R" {
                    rules.push(Box::new(reject));
                } else if !rule.contains(':') {  // returns other Rule's label directly
                    let rule = rule.to_owned();
                    rules.push(Box::new(move |_| Some(rule.clone())));
                } else {
                    let mut split = rule.split(':');
                    let condition = split.next().unwrap();
                    let variable = condition.chars().next().unwrap();
                    let operation = condition.chars().nth(1).unwrap();
                    let compare_value = condition[2..].parse::<u32>().unwrap();
                    let destination = split.next().unwrap().to_owned();

                    rules.push(Box::new(move |part: &Part| {
                        let value = match variable {
                            'x' => part.x,
                            'm' => part.m,
                            'a' => part.a,
                            's' => part.s,
                            _ => panic!("Invalid variable"),
                        };

                        match operation {
                            '<' => {
                                if value < compare_value {
                                    Some(destination.clone())
                                } else {
                                    None
                                }
                            }
                            '>' => {
                                if value > compare_value {
                                    Some(destination.clone())
                                } else {
                                    None
                                }
                            }
                            _ => panic!("Invalid operation"),
                        }
                    }));
                }
            });

            workflows.insert(name, rules);
        }
    }

    (parts, workflows)
}

fn apply_workflow(part: &Part, workflow: &Workflow) -> String {
    for rule in workflow {
        if let Some(next_rule) = rule(part) {
            return next_rule;
        }
    }
    unreachable!()  // final rule of workflow is always a label/reject/accept
}

fn solve(parts: &[Part], workflows: &Workflows) -> u32 {
    parts.iter().filter(|part| {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let next_rule = apply_workflow(part, workflow);
            if next_rule == "A" {
                return true;
            } else if next_rule == "R" {
                return false;
            }
            workflow = workflows.get(&next_rule).unwrap();
        }
    }).fold(0, |acc, f| {
        acc + f.x + f.m + f.a + f.s
    })
}

fn main() {
    let _input = read_lines_as_vec("inputs/19.txt").unwrap();
    let _example = r#"px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}

    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}"#.lines().map(|s| s.to_string()).collect::<Vec<_>>();

    let (parts, worflows) = parse(&_input);
    let r1 = solve(&parts, &worflows);
    println!("Part 1: {}", r1);
}
