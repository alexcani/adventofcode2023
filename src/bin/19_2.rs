use advent_of_code_2023::read_lines_as_vec;
use std::collections::HashMap;
use regex::Regex;

// A range is a closed interval [a, b]
#[derive(Debug, Clone, Copy)]
struct Range_<const MIN: u32, const MAX: u32> {
    a: u32,
    b: u32,
}

type Range = Range_<1, 4000>;

impl<const MIN: u32, const MAX: u32> Range_<MIN, MAX> {
    fn default() -> Self {
        Range_ {
            a: MIN,
            b: MAX,
        }
    }

    fn new_a(a: u32) -> Self {
        if  a < MIN || a > MAX {
            panic!("Invalid range");
        }

        Range_ {
            a,
            b: MAX
        }
    }

    fn new_b(b: u32) -> Self {
        if  b < MIN || b > MAX {
            panic!("Invalid range");
        }

        Range_ {
            a: MIN,
            b
        }
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.a > other.b || other.a > self.b {
            None
        } else {
            Some(Range_ {
                a: self.a.max(other.a),
                b: self.b.min(other.b),
            })
        }
    }

    fn subtraction(&self, other: &Self) -> Option<Self> {
        self.intersection(&other.complement())
    }

    // To be used on ranged where a == Min xor b == Max only
    fn complement(&self) -> Self {
        if self.a == MIN && self.b == MAX{
            panic!("Invalid complement input")
        } else if self.a == MIN {
            Range_ {
                a: self.b + 1,
                b: MAX,
            }
        } else if self.b == MAX {
            Range_ {
                a: MIN,
                b: self.a - 1,
            }
        } else {
            panic!("Invalid complement input 2")
        }
    }

    fn length(&self) -> u64 {
        self.b as u64 - self.a as u64 + 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl Part {
    fn combinations(&self) -> u64 {
        self.x.length() * self.m.length() * self.a.length() * self.s.length()
    }
}

#[derive(Debug)]
struct Rule {
    variable: Option<String>,
    operation: Option<char>,
    value: Option<u32>,
    destination: String,
}

type Workflow = Vec<Rule>;
type Workflows = HashMap<String, Workflow>; // label -> rules

fn parse(input: &[String]) -> Workflows {
    let mut workflows = HashMap::new();
    let workflow_re = Regex::new(r"^(\w+)\{(.*)\}$").unwrap();

    for line in input {
        let line = line.trim();
        if line.is_empty() || line.starts_with('{') {
            continue;
        }

        // Parse workflow
        let mut workflow: Workflow = Vec::new();
        let captures = workflow_re.captures(line).unwrap();

        let label = captures.get(1).unwrap().as_str().to_string();
        let rules_str = captures.get(2).unwrap().as_str();
        rules_str.split(',').for_each(|rule| {
            if !rule.contains(':') {
                // returns other Rule's label directly, A or R
                workflow.push(Rule {
                    variable: None,
                    operation: None,
                    value: None,
                    destination: rule.to_owned(),
                });
            } else {
                let mut split = rule.split(':');
                let condition = split.next().unwrap();

                let variable = condition.chars().next().unwrap();
                let operation = condition.chars().nth(1).unwrap();
                let compare_value = condition[2..].parse::<u32>().unwrap();

                let destination = split.next().unwrap().to_owned();

                workflow.push(Rule {
                    variable: Some(variable.to_string()),
                    operation: Some(operation),
                    value: Some(compare_value),
                    destination,
                });
            }
        });

        workflows.insert(label, workflow);
    }

    workflows
}

// Recursively calculate the possible part ranges that will yield in approved parts
fn calculate(workflow: &Workflow, part_range: Part, workflows: &Workflows) -> Vec<Part> {
    let mut result = Vec::new();
    let mut current_parts_range = part_range;

    for rule in workflow {
        if rule.variable.is_none() {
            // A, R or other Rule's label directly
            if rule.destination == "A" {  // this part range got approved
                result.push(current_parts_range);
            } else if rule.destination == "R" {  // this part range got rejected
                break;
            } else {
                // other Rule's label directly
                let next_workflow = workflows.get(&rule.destination).unwrap();
                result.append(&mut calculate(next_workflow, current_parts_range, workflows));
            }

            break;
        }

        let variable = rule.variable.as_ref().unwrap();
        let operation = rule.operation.unwrap();
        let value = rule.value.unwrap();

        // x > 1024
        let comparison_range = match operation {
            '>' => Range::new_a(value + 1),
            '<' => Range::new_b(value - 1),
            _ => panic!("Invalid operation"),
        };

        // Parts to be sent to the next workflow
        let mut passing_part_range = current_parts_range;
        match variable.as_str() {
            "x" => {
                if let Some(passing_range) = current_parts_range.x.intersection(&comparison_range) {
                    passing_part_range.x = passing_range;
                    current_parts_range.x = current_parts_range.x.subtraction(&comparison_range).unwrap();  // what didn't pass
                } else {
                    continue;  // didn't pass, skip to next rule
                }
            },
            "m" => {
                if let Some(passing_range) = current_parts_range.m.intersection(&comparison_range) {
                    passing_part_range.m = passing_range;
                    current_parts_range.m = current_parts_range.m.subtraction(&comparison_range).unwrap();
                } else {
                    continue;
                }
            },
            "a" => {
                if let Some(passing_range) = current_parts_range.a.intersection(&comparison_range) {
                    passing_part_range.a = passing_range;
                    current_parts_range.a = current_parts_range.a.subtraction(&comparison_range).unwrap();
                } else {
                    continue;
                }
            },
            "s" => {
                if let Some(passing_range) = current_parts_range.s.intersection(&comparison_range) {
                    passing_part_range.s = passing_range;
                    current_parts_range.s = current_parts_range.s.subtraction(&comparison_range).unwrap();
                } else {
                    continue;
                }
            },
            _ => panic!("Invalid variable"),
        }

        // Recursively calculate the possible part ranges that will yield in approved parts
        if &rule.destination == "A" {
            result.push(passing_part_range);
        } else if &rule.destination == "R" {
            continue;
        } else {
            let next_workflow = workflows.get(&rule.destination).unwrap();
            result.append(&mut calculate(next_workflow, passing_part_range, workflows));
        }
    }

    result
}

fn solve(input: &Workflows) -> u64 {
    let part = Part {
        x: Range::default(),
        m: Range::default(),
        a: Range::default(),
        s: Range::default(),
    };

    let ranges = calculate(input.get("in").unwrap(), part, input);
    ranges.iter().map(|r| r.combinations()).sum()
}

fn main() {
    let _lines = read_lines_as_vec("inputs/19.txt").unwrap();
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

    let workflows = parse(&_lines);
    let r2 = solve(&workflows);
    println!("Part 2: {}", r2);
}
