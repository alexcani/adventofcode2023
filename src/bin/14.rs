use std::collections::HashMap;

use advent_of_code_2023::read_lines_as_vec;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Ground,
    Ball,
    Rock,
}

// Takes the map and a direction, rolling all the balls in that direction
fn step(map: &[Vec<Tile>], direction: &Direction) -> Vec<Vec<Tile>> {
    let mut map = map.to_vec();

    // Starting coordinates of the 2 tiles of interest
    let (mut row1, mut col1, mut row2, mut col2)= match direction {
        Direction::North => (0, 0, 1, 0),
        Direction::South => (map.len() - 1, 0, map.len() - 2, 0),
        Direction::East => (0, map[0].len() - 1, 0, map[0].len() - 2),
        Direction::West => (0, 0, 0, 1),
    };

    loop {
        // Given the 2 tiles of interest, roll the ball if possible
        if let (Tile::Ground, Tile::Ball) = (map[row1][col1], map[row2][col2]) {
            map[row1][col1] = Tile::Ball;
            map[row2][col2] = Tile::Ground;
        }

        // Increment tile coordinates based on distance
        row1 = row2;
        col1 = col2;

        match direction {
            Direction::North => {
                if row2 == map.len() - 1 {  // Move to next column
                    if col2 == map[row2].len() - 1 {  // Reached end of map
                        break;
                    } else {
                        row1 = 0;
                        col1 += 1;
                        row2 = 1;
                        col2 = col1;
                    }
                } else {
                    row2 += 1;
                }
            },
            Direction::South => {
                if row2 == 0 {  // Move to next column
                    if col2 == map[row2].len() - 1 {  // Reached end of map
                        break;
                    } else {
                        row1 = map.len() - 1;
                        col1 += 1;
                        row2 = map.len() - 2;
                        col2 = col1;
                    }
                } else {
                    row2 -= 1;
                }
            },
            Direction::East => {
                if col2 == 0 {  // Move to next row
                    if row2 == map.len() - 1 {  // Reached end of map
                        break;
                    } else {
                        row1 += 1;
                        col1 = map[0].len() - 1;
                        row2 = row1;
                        col2 = map[0].len() - 2;
                    }
                } else {
                    col2 -= 1;
                }
            },
            Direction::West => {
                if col2 == map[0].len() - 1 {  // Move to next row
                    if row2 == map.len() - 1 {  // Reached end of map
                        break;
                    } else {
                        row1 += 1;
                        col1 = 0;
                        row2 = row1;
                        col2 = 1;
                    }
                } else {
                    col2 += 1;
                }
            },
        }
    }

    map
}

fn parse(lines: &[String]) -> Vec<Vec<Tile>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Ground,
                    'O' => Tile::Ball,
                    '#' => Tile::Rock,
                    _ => panic!("Invalid character in map: {}", c),
                })
                .collect()
        })
        .collect()
}

fn solve(lines: &[String]) -> (u32, u32) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    let mut map = parse(lines);
    // Part 1 - Roll all balls north
    loop {
        let new_map = step(&map, &Direction::North);
        if new_map == map {
            // Roll until the map doesn't change
            break;
        }
        map = new_map;
    }

    // Calculate result
    for (index, line) in map.iter().rev().enumerate() {
        // Each ball in this line is worth index+1 points
        for tile in line {
            if *tile == Tile::Ball {
                result_1 += index as u32 + 1;
            }
        }
    }

    // Part 2 - Perfom 1_000_000_000 cycles, each cycle being a full roll North->West->South->East
    let mut map = parse(lines);
    let mut cache = HashMap::new();
    for i in 0..1_000_000_000 {
        for direction in Direction::iter() {
            loop {
                let new_map = step(&map, &direction);
                if new_map == map {
                    // Roll until the map doesn't change
                    break;
                }
                map = new_map;
            }
        }

        // Cache the map every cycle to see if we've already seen it
        if let Some(j) = cache.insert(map.clone(), i) {
            // We've seen this map before, which means iteration i ended a cycle from j to i-1
            let cycle_length = i - j;
            let remaining_iterations = 1_000_000_000 - i - 1;
            let remaining_iterations = remaining_iterations % cycle_length;
            let original_index = remaining_iterations + j;  // add the j cycles before we entered the loop
            map = cache.iter().find(|(_, &v)| v == original_index).unwrap().0.to_vec();
            break;
        }
    }

    // Calculate result
    for (index, line) in map.iter().rev().enumerate() {
        // Each ball in this line is worth index+1 points
        for tile in line {
            if *tile == Tile::Ball {
                result_2 += index as u32 + 1;
            }
        }
    }

    (result_1, result_2)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/14.txt").unwrap();

    let _example: Vec<String> = [
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();

    let (r1, r2) = solve(&_lines);
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
