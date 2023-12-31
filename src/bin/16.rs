use std::time::Instant;

use advent_of_code_2023::read_lines_as_vec;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Default)]
enum TileType {
    #[default] Empty,
    VSplitter,
    HSplitter,
    SlashMirror,
    BackslashMirror,
}

#[derive(Clone, Default)]
struct Tile {
    tile_type: TileType,
    energized_from: Vec<Direction>,
}

fn process(grid: &[Vec<Tile>], initial_beam: (usize, usize, Direction)) -> u32 {
    let mut grid = grid.to_owned();
    let mut beams = Vec::new();
    beams.push(initial_beam);  // initial beam

    loop {
        if beams.is_empty() {
            break;
        }

        let beam = beams.pop().unwrap();

        let mut row = beam.0;
        let mut col = beam.1;
        let mut direction = beam.2;

        loop {
            let tile = &mut grid[row][col];
            if tile.energized_from.contains(&direction) {  // we've been here before
                break;
            }
            tile.energized_from.push(direction);

            match &tile.tile_type {
                TileType::Empty => (),  // Continue in same direction
                TileType::VSplitter => {  // found |
                    match &direction {
                        Direction::Down | Direction::Up => (),  // Continue in same direction
                        _ => {
                            // Split
                            if row > 0 {
                                beams.push((row - 1, col, Direction::Up));
                            }
                            if row < grid.len() - 1 {
                                beams.push((row + 1, col, Direction::Down));
                            }
                            break;
                        },
                    }
                },
                TileType::HSplitter => {  // found -
                    match &direction {
                        Direction::Left | Direction::Right => (),  // Continue in same direction
                        _ => {
                            // Split
                            if col > 0 {
                                beams.push((row, col - 1, Direction::Left));
                            }
                            if col < grid[0].len() - 1 {
                                beams.push((row, col + 1, Direction::Right));
                            }
                            break;
                        },
                    }
                },
                TileType::SlashMirror => {  // Found /
                        match &direction {
                            Direction::Up => direction = Direction::Right,
                            Direction::Down => direction = Direction::Left,
                            Direction::Left => direction = Direction::Down,
                            Direction::Right => direction = Direction::Up,
                        }
                    },
                    TileType::BackslashMirror => {  // Found \
                        match &direction {
                            Direction::Up => direction = Direction::Left,
                            Direction::Down => direction = Direction::Right,
                            Direction::Left => direction = Direction::Up,
                            Direction::Right => direction = Direction::Down,
                        }
                },
            }

            match direction {
                Direction::Up => {
                    if row > 0 {
                        row -= 1;
                    } else {
                        break;
                    }
                },
                Direction::Down => {
                    if row < grid.len() - 1 {
                        row += 1;
                    } else {
                        break;
                    }
                },
                Direction::Left => {
                    if col > 0 {
                        col -= 1;
                    } else {
                        break;
                    }
                },
                Direction::Right => {
                    if col < grid[0].len() - 1 {
                        col += 1;
                    } else {
                        break;
                    }
                },
            }
        }
    }

    // Return number of energized tiles
    grid.iter().flatten().filter(|t| !t.energized_from.is_empty()).count() as u32
}

fn solve(lines: &[String]) -> (u32, u32) {
    let mut grid = vec![vec![Tile::default(); lines[0].len()]; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x].tile_type = match c {
                '|' => TileType::VSplitter,
                '-' => TileType::HSplitter,
                '/' => TileType::SlashMirror,
                '\\' => TileType::BackslashMirror,
                _ => TileType::Empty,
            };
        }
    }

    let part1 = process(&grid, (0, 0, Direction::Right));

    let start = Instant::now();
    // Part 2 - Beam can start from any point in the border, find maximum value of energied tiles
    let top = (0..grid[0].len()).map(|c| (0, c, Direction::Down));  // Along top border
    let bottom = (0..grid[0].len()).map(|c| (grid.len() - 1, c, Direction::Up));  // Along bottom border
    let left = (0..grid.len()).map(|r| (r, 0, Direction::Right));  // Along left border
    let right = (0..grid.len()).map(|r| (r, grid[0].len() - 1, Direction::Left));  // Along right border

    let part2 = top.chain(bottom).chain(left).chain(right).map(|b| process(&grid, b)).max().unwrap();
    let end = start.elapsed();
    println!("Part 2 took: {:?}", end);
    (part1, part2)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/16.txt").unwrap();
    let _example = r#".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."#.lines().map(|s| s.trim().to_owned()).collect::<Vec<_>>();

    let (r1, r2) = solve(&_lines);
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
