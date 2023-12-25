use std::collections::HashSet;

use advent_of_code_2023::read_lines_as_vec;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, PartialEq)]
enum Pipes {
    G,  // Ground / No pipe
    V,  // Vertical
    H,  // Horizontal
    F,  // South-East
    L,  // North-East
    J,  // North-West
    SW, // South-West (7)
    S,  // Animal / Unknown pipe
}

#[derive(EnumIter, Debug)]
enum Directions {
    N,
    S,
    E,
    W,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Coord(usize, usize);

// Given a coordinate and a direction, check if a connection exists from this pipe into that direction
fn neighbour_connects(pipes: &Vec<Vec<Pipes>>, coord: &Coord, direction: &Directions) -> bool {
    let (row, column) = (coord.0, coord.1);

    match direction {
        Directions::N => {
            if row == 0 {
                return false;
            }

            matches!(pipes[row - 1][column], Pipes::V | Pipes::F | Pipes::SW | Pipes::S)
        }
        Directions::S => {
            if row == pipes.len() - 1 {
                return false;
            }

            matches!(pipes[row + 1][column], Pipes::V | Pipes::L | Pipes::J | Pipes::S)
        }
        Directions::E => {
            if column == pipes[0].len() - 1 {
                return false;
            }

            matches!(pipes[row][column + 1], Pipes::H | Pipes::SW | Pipes::J | Pipes::S)
        }
        Directions::W => {
            if column == 0 {
                return false;
            }

            matches!(pipes[row][column - 1], Pipes::H | Pipes::F | Pipes::L | Pipes::S)
        }
    }
}

// From current coord, follow the pipe in the given direction, returning the next coordinate and the direction we're facing
fn follow_pipe(pipes: &Vec<Vec<Pipes>>, current_coord: &Coord, direction: &Directions) -> (Coord, Directions) {
    let (row, column) = (current_coord.0, current_coord.1);

    match direction {
        Directions::N => {
            let next_coord = Coord(row - 1, column);
            match pipes[row - 1][column] {
                Pipes::V => (next_coord, Directions::N),
                Pipes::F => (next_coord, Directions::E),
                Pipes::SW => (next_coord, Directions::W),
                Pipes::S => (next_coord, Directions::S),
                _ => panic!("Can't go north"),
            }
        }
        Directions::S => {
            let next_coord = Coord(row + 1, column);
            match pipes[row + 1][column] {
                Pipes::V => (next_coord, Directions::S),
                Pipes::L => (next_coord, Directions::E),
                Pipes::J => (next_coord, Directions::W),
                Pipes::S => (next_coord, Directions::S),
                _ => panic!("Can't go south"),
            }
        }
        Directions::E => {
            let next_coord = Coord(row, column + 1);
            match pipes[row][column + 1] {
                Pipes::H => (next_coord, Directions::E),
                Pipes::SW => (next_coord, Directions::S),
                Pipes::J => (next_coord, Directions::N),
                Pipes::S => (next_coord, Directions::S),
                _ => panic!("Can't go east"),
            }
        }
        Directions::W => {
            let next_coord = Coord(row, column - 1);
            match pipes[row][column - 1] {
                Pipes::H => (next_coord, Directions::W),
                Pipes::F => (next_coord, Directions::S),
                Pipes::L => (next_coord, Directions::N),
                Pipes::S => (next_coord, Directions::S),
                _ => panic!("Can't go east"),
            }
        }
    }
}

fn figure_s_type(nodes: &Vec<Vec<Pipes>>, s_coord: &Coord, neighbour_direction: &Directions) -> Pipes {
    // We know there is a connecting neighbour in the given direction, so no we test where the other neighbour is
    // to find out the type of S
    match neighbour_direction {
        Directions::N => {
            if neighbour_connects(&nodes, &s_coord, &Directions::E) {
                Pipes::L
            } else if neighbour_connects(&nodes, &s_coord, &Directions::W) {
                Pipes::J
            } else {
                Pipes::V
            }
        }
        Directions::S => {
            if neighbour_connects(&nodes, &s_coord, &Directions::E) {
                Pipes::F
            } else if neighbour_connects(&nodes, &s_coord, &Directions::W) {
                Pipes::SW
            } else {
                Pipes::V
            }
        }
        Directions::E => {
            if neighbour_connects(&nodes, &s_coord, &Directions::N) {
                Pipes::L
            } else if neighbour_connects(&nodes, &s_coord, &Directions::S) {
                Pipes::F
            } else {
                Pipes::H
            }
        }
        Directions::W => {
            if neighbour_connects(&nodes, &s_coord, &Directions::N) {
                Pipes::J
            } else if neighbour_connects(&nodes, &s_coord, &Directions::S) {
                Pipes::SW
            } else {
                Pipes::H
            }
        }
    }
}

fn calculate_result(input: &[String]) -> (u64, u64) {
    let rows = input.len();
    let columns = input[0].len();

    let mut map = vec![vec![Pipes::G; columns]; rows];

    let mut s_coord = Coord(0, 0);
    for (row_index, row) in input.iter().enumerate() {
        for (column_index, character) in row.chars().enumerate() {
            map[row_index][column_index] = match character {
                '.' => Pipes::G,
                '|' => Pipes::V,
                '-' => Pipes::H,
                'F' => Pipes::F,
                'L' => Pipes::L,
                'J' => Pipes::J,
                '7' => Pipes::SW,
                'S' => {
                    s_coord.0 = row_index;
                    s_coord.1 = column_index;
                    Pipes::S
                }
                _ => panic!("Unknown character: {}", character),
            };
        }
    }

    // Assume that S only has 2 connecting neighbours (this is the case for the input)
    // This way we only need to find the first neighbour that connects to S and follow the pipe until we're back on S

    // Store the coordinates that belong to the loop
    let mut loop_coords = HashSet::new();
    loop_coords.insert(s_coord.clone());

    // Find first connecting neighbour
    let mut current_direction = Directions::iter().find(|d| neighbour_connects(&map, &s_coord, d)).unwrap();
    let s_tile_type = figure_s_type(&map, &s_coord, &current_direction);
    let mut current_coord = s_coord;

    let mut loop_length = 1;
    // Follow the pipes until we're back at S
    loop {
        (current_coord, current_direction) = follow_pipe(&map, &current_coord, &current_direction);
        if let Pipes::S = map[current_coord.0][current_coord.1] {
            break;
        }

        loop_coords.insert(current_coord.clone());
        loop_length += 1;
    }

    // Count the number of tiles inside the loop
    let mut tiles_inside = 0;
    let mut inside_loop = false;
    let mut inner_group = false;
    let mut inner_initial_pipe = Pipes::G;

    // Traverse the map horizontally, starting from the top left
    // Vertical tiles count as a wall, as well as F--J and L--7 since they run "vertically"
    // Horizontal tiles, F--7 and L--J don't count as walls, since they run horizontally
    // When we meet a wall, we flip the inside loop bit
    for (row_index, row) in map.iter().enumerate() {
        for (column_index, _) in row.iter().enumerate() {
            let mut tile_type = &map[row_index][column_index];
            if tile_type == &Pipes::S {
                tile_type = &s_tile_type;
            }

            let is_tile_in_loop = loop_coords.contains(&Coord(row_index, column_index));

            // If the tile is not part of the loop we count it according to the flag
            if !is_tile_in_loop {
                if inside_loop {
                    tiles_inside += 1;
                }
            } else {
                // If it is part of the loop we need to figure out if we should flip or not the flag
                match tile_type {
                    Pipes::V => {  // vertical walls always flip the flag
                        inside_loop = !inside_loop;
                    }
                    Pipes::F => {  // mark that we found an F to be resolved by the next 7 or J
                        inner_group = true;
                        inner_initial_pipe = Pipes::F;
                    }
                    Pipes::L => {  // mark that we found an L to be resolved by the next 7 or J
                        inner_group = true;
                        inner_initial_pipe = Pipes::L;
                    }
                    Pipes::J => {  // if we find a J we take the decision based on the previous F or L
                        if inner_group && inner_initial_pipe == Pipes::F {  // F--J
                            inside_loop = !inside_loop;
                        }
                        inner_group = false;
                    }
                    Pipes::SW => {  // if we find a 7 we take the decision based on the previous F or L
                        if inner_group && inner_initial_pipe == Pipes::L {  // L--7
                            inside_loop = !inside_loop;
                        }
                        inner_group = false;
                    }

                    _ => {  // all other tiles don't affect the flag
                        continue;
                    }
                }
            }
        }
    }

    (loop_length / 2, tiles_inside)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/10.txt").unwrap();

    let _example = [
        "7-F7-",
        ".FJ|7",
        "SJLL7",
        "|F--J",
        "LJ.LJ",
    ].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let (max_distance, tiles_inside) = calculate_result(&_lines);
    println!("Max distance: {}", max_distance);
    println!("Tiles inside: {}", tiles_inside);
}
