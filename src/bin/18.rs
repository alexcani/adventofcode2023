use advent_of_code_2023::read_lines_as_vec;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    steps: i64,
    color: [u8; 3], // RGB
}

fn solve(input: &[Instruction]) -> u64 {
    let mut vertices = Vec::new();

    let mut current_vertex = (0, 0, [0; 3]); // (x, y, (r, g, b)) initial vertex
    for instruction in input {
        let next_vertex = match instruction.direction {
            Direction::Up => {
                (current_vertex.0, current_vertex.1 + instruction.steps, instruction.color)
            }
            Direction::Down => {
                (current_vertex.0, current_vertex.1 - instruction.steps, instruction.color)
            }
            Direction::Left => {
                (current_vertex.0 - instruction.steps, current_vertex.1, instruction.color)
            }
            Direction::Right => {
                (current_vertex.0 + instruction.steps, current_vertex.1, instruction.color)
            }
        };

        vertices.push(current_vertex);
        current_vertex = next_vertex;
    }
    vertices.push(current_vertex);

    // Shoelace formula for calculating area of polygon given vertices
    // Also calculate perimeter
    let mut perimeter = 0;
    let mut area = 0;
    for (pn, pn1) in vertices.iter().tuple_windows() {
        area += (pn.1+pn1.1)*(pn.0-pn1.0);
        perimeter += (pn.0 - pn1.0).unsigned_abs() + (pn.1 - pn1.1).unsigned_abs();
    }
    let area = (area.abs()/2) as u64;

    // Apply Pick's theorem to the area of the polygon
    // A = i + b/2 - 1
    // i = A - b/2 + 1
    let interior_points = area - perimeter/2 + 1;

    // Total number of point is i + b
    interior_points + perimeter
}

fn parse(input: &[String]) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input {
        let mut split = line.split_whitespace();
        let direction = match split.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let steps = split.next().unwrap().parse::<i64>().unwrap();
        // Parse (#rrggbb) into [r, g, b]
        let color = split
            .next()
            .unwrap()
            .trim_start_matches("(#")
            .trim_end_matches(")")
            .split("")
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|c| u8::from_str_radix(&c.join(""), 16).unwrap())
            .collect::<Vec<_>>();

        instructions.push(Instruction {
            direction,
            steps,
            color: [color[0], color[1], color[2]],
        });
    }

    instructions
}

fn parse2(input: &[String]) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in input {
        let (steps, direction) = line.split("(#").nth(1).unwrap().trim_end_matches(')').split_at(5);
        let steps = i64::from_str_radix(steps, 16).unwrap();
        let direction = match direction {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Invalid direction"),
        };

        instructions.push(Instruction {
            direction,
            steps,
            color: [0, 0, 0],
        });
    }

    instructions
}

fn main() {
    let _lines = read_lines_as_vec("inputs/18.txt").unwrap();

    let _example = r#"R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)"#.lines().map(|l| l.to_string()).collect::<Vec<_>>();

    let _example_2 = r#"R 6 (#000000)
    D 2 (#000000)
    L 6 (#000000)
    U 2 (#000000)"#.lines().map(|l| l.to_string()).collect::<Vec<_>>();

    let r1 = solve(&parse(&_lines));
    let r2 = solve(&parse2(&_lines));
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
