use advent_of_code_2023::read_lines_as_vec;
use pathfinding::prelude::dijkstra_all;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    column: usize,
}

fn parse(lines: &[String]) -> (Vec<Vec<bool>>, Node) {
    // true is ground
    // false is rocks
    let mut grid = vec![vec![true; lines[0].len()]; lines.len()];
    let mut start_node = Node {
        row: 0,
        column: 0,
    };

    for (row, line) in lines.iter().enumerate() {
        for (column, c) in line.chars().enumerate() {
            match c {
                '.' => grid[row][column] = true,
                '#' => grid[row][column] = false,
                'S' => {
                    grid[row][column] = true;
                    start_node.row = row;
                    start_node.column = column;
                }
                _ => panic!("Invalid character"),
            }
        }
    }

    (grid, start_node)
}

// Returns a map of all reachable nodes in the map and their distance to the center
fn solve(grid: &[Vec<bool>], start_node: Node) -> HashMap<Node, u32> {
    let mut solution = dijkstra_all(&start_node, |node| {
        let mut neighbors = Vec::new();
        if node.column > 0 {
            // left
            if grid[node.row][node.column - 1] {
                neighbors.push((
                    Node {
                        row: node.row,
                        column: node.column - 1,
                    },
                    1,
                ));
            }
        }
        if node.column < grid[0].len() - 1 {
            // right
            if grid[node.row][node.column + 1] {
                neighbors.push((
                    Node {
                        row: node.row,
                        column: node.column + 1,
                    },
                    1,
                ));
            }
        }
        if node.row > 0 {
            // up
            if grid[node.row - 1][node.column] {
                neighbors.push((
                    Node {
                        row: node.row - 1,
                        column: node.column,
                    },
                    1,
                ));
            }
        }
        if node.row < grid.len() - 1 {
            // down
            if grid[node.row + 1][node.column] {
                neighbors.push((
                    Node {
                        row: node.row + 1,
                        column: node.column,
                    },
                    1,
                ));
            }
        }
        neighbors
    });

    solution.insert(start_node, (start_node, 0));  // insert the start node

    solution.into_iter().map(|(node, (_, cost))| {
        (node, cost)
    }).collect::<HashMap<Node, u32>>()
}

fn part_1(solution: &HashMap<Node, u32>, steps: u32) -> u32 {
    // Find nodes with same parity as steps
    let parity = steps % 2;
    solution.iter().filter(|(_, cost) | **cost % 2 == parity && **cost <= steps).count() as u32
}

// Based on https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
fn part_2(solution: &HashMap<Node, u32>, steps: u32, square_dimension: u32) -> u64 {
    let parity = steps % 2;

    let even_nodes = solution.iter().filter(|(_, cost)| **cost % 2 == 0).count() as u64;
    let odd_nodes = solution.iter().filter(|(_, cost)| **cost % 2 == 1).count() as u64;

    // n is the number of copies of the map we can fit in the steps
    let n = (steps - square_dimension / 2) / square_dimension;
    let n = n as u64;

    let (n_full_even_squares, n_full_odd_squares) = if n % 2 == 0 {
        // even n means there are (n+1)^2 squares with same parity as starting square and n² squares with opposite parity
        if parity == 0 {  // starting square is even
            ((n+1).pow(2), n.pow(2))
        } else {  // starting square is odd
            (n.pow(2), (n+1).pow(2))
        }
    } else {
        // odd n means there are n² squares with same parity as starting square and (n+1)^2 squares with opposite parity
        if parity == 0 {  // starting square is even
            (n.pow(2), (n+1).pow(2))
        } else {  // starting square is odd
            ((n+1).pow(2), n.pow(2))
        }
    };

    let even_half_nodes = solution.iter().filter(|(_, cost)| **cost % 2 == 0 && **cost > square_dimension/2).count() as u64;
    let odd_half_nodes = solution.iter().filter(|(_, cost)| **cost % 2 == 1 && **cost > square_dimension/2).count() as u64;

    // Number of each half nodes depend on parity
    let (n_even_hal, n_odd_half) = if parity == 0 {
        (n+1, n)
    } else {
        (n, n+1)
    };

    // Wheter we subtract or add the half nodes depends on parity too
    if parity == 0 {
        even_nodes * n_full_even_squares + odd_nodes * n_full_odd_squares - n_even_hal * even_half_nodes + n_odd_half * odd_half_nodes
    } else {
        even_nodes * n_full_even_squares + odd_nodes * n_full_odd_squares + n_even_hal * even_half_nodes - n_odd_half * odd_half_nodes
    }
}

fn main() {
    let _lines = read_lines_as_vec("inputs/21.txt").unwrap();

    let _example = r#"...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ..........."#
        .lines()
        .map(|l| l.trim().to_owned())
        .collect::<Vec<_>>();

    let (grid, start_node) = parse(&_lines);
    let solution = solve(&grid, start_node);

    let result = part_1(&solution, 64);
    println!("Part 1: {}", result);

    let result = part_2(&solution, 26501365, grid.len() as u32);
    println!("Part 2: {}", result);
}
