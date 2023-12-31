use advent_of_code_2023::read_lines_as_vec;
use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    row: usize,
    col: usize,
    forward_streak: u32,
    direction: Direction,
}

fn solve(input: &[String], min_streak: u32, max_streak: u32) -> u32 {
    let grid = input
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    // Calculate successors given a node
    let successors = |node: &Node| {
        let mut successors = Vec::new();
        let row = node.row;
        let col = node.col;

        // Based on current direction, calculate the neighbors to left, right and front
        match node.direction {
            Direction::Up => {
                if col > 0 && node.forward_streak >= min_streak {
                    // we can move left
                    successors.push(Node {
                        row,
                        col: col - 1,
                        forward_streak: 1,
                        direction: Direction::Left,
                    });
                }
                if col < grid[0].len() - 1 && node.forward_streak >= min_streak {
                    // we can move right
                    successors.push(Node {
                        row,
                        col: col + 1,
                        forward_streak: 1,
                        direction: Direction::Right,
                    });
                }
                if row > 0 && node.forward_streak != max_streak {
                    // we can move forward
                    successors.push(Node {
                        row: row - 1,
                        col,
                        forward_streak: node.forward_streak + 1,
                        direction: Direction::Up,
                    });
                }
            }
            Direction::Down => {
                if col > 0 && node.forward_streak >= min_streak{
                    // we can move left
                    successors.push(Node {
                        row,
                        col: col - 1,
                        forward_streak: 1,
                        direction: Direction::Left,
                    });
                }
                if col < grid[0].len() - 1 && node.forward_streak >= min_streak {
                    // we can move right
                    successors.push(Node {
                        row,
                        col: col + 1,
                        forward_streak: 1,
                        direction: Direction::Right,
                    });
                }
                if row < grid.len() - 1 && node.forward_streak != max_streak {
                    // we can move forward
                    successors.push(Node {
                        row: row + 1,
                        col,
                        forward_streak: node.forward_streak + 1,
                        direction: Direction::Down,
                    });
                }
            }
            Direction::Left => {
                if row > 0 && node.forward_streak >= min_streak {
                    // we can move up
                    successors.push(Node {
                        row: row - 1,
                        col,
                        forward_streak: 1,
                        direction: Direction::Up,
                    });
                }
                if row < grid.len() - 1 && node.forward_streak >= min_streak {
                    // we can move down
                    successors.push(Node {
                        row: row + 1,
                        col,
                        forward_streak: 1,
                        direction: Direction::Down,
                    });
                }
                if col > 0 && node.forward_streak != max_streak {
                    // we can move forward
                    successors.push(Node {
                        row,
                        col: col - 1,
                        forward_streak: node.forward_streak + 1,
                        direction: Direction::Left,
                    });
                }
            }
            Direction::Right => {
                if row > 0 && node.forward_streak >= min_streak {
                    // we can move up
                    successors.push(Node {
                        row: row - 1,
                        col,
                        forward_streak: 1,
                        direction: Direction::Up,
                    });
                }
                if row < grid.len() - 1 && node.forward_streak >= min_streak {
                    // we can move down
                    successors.push(Node {
                        row: row + 1,
                        col,
                        forward_streak: 1,
                        direction: Direction::Down,
                    });
                }
                if col < grid[0].len() - 1 && node.forward_streak != max_streak {
                    // we can move forward
                    successors.push(Node {
                        row,
                        col: col + 1,
                        forward_streak: node.forward_streak + 1,
                        direction: Direction::Right,
                    });
                }
            }
        }

        successors
            .iter()
            .map(|n| {
                // Get cost of node
                let cost = grid[n.row][n.col];
                (*n, cost)
            })
            .collect::<Vec<_>>()
    };

    let result = dijkstra(
        &Node {
            row: 0,
            col: 0,
            forward_streak: 0,
            direction: Direction::Right,
        },
        successors,
        |n| n.row == grid.len() - 1 && n.col == grid[0].len() - 1 && n.forward_streak >= min_streak,
    ).unwrap();

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if let Some(node) = result.0.iter().find(|n| n.row == y && n.col == x) {
                match node.direction {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                }
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();

    result.1
}

fn main() {
    let _lines = read_lines_as_vec("inputs/17.txt").unwrap();
    let _example = r#"2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533"#
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();

    let _example_2 = r#"111111111111
    999999999991
    999999999991
    999999999991
    999999999991"#.lines().map(|l| l.trim().to_string()).collect::<Vec<String>>();

    let r1 = solve(&_example, 0, 3);
    let r2 = solve(&_lines, 4, 10);

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
