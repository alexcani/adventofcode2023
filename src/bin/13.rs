use advent_of_code_2023::read_lines_as_vec;
use itertools::Itertools;

fn transpose(map: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..map[0].len() {
        // columns
        let mut new_row = String::new();
        for row in map {
            // rows
            new_row.push(row.chars().nth(i).unwrap());
        }
        result.push(new_row);
    }
    result
}

// Find the number of rows above the reflection line
fn solve_map(map: &[String]) -> Vec<u32> {
    // Iterate over pairs of lines until we find identical ones
    map.iter()
        .tuple_windows()
        .enumerate()
        .filter_map(|(row_idx, (row_1, row_2))| {
            if row_1 != row_2 {
                return None;
            }

            // Interleave the forward iterator with the backward iterator to compare the corresponding lines
            // with respect to the reflection line
            if map
                .iter()
                .skip(row_idx + 1)
                .interleave_shortest(map.iter().rev().skip(map.len() - row_idx - 1))
                .chunks(2)
                .into_iter()
                .all(|chunk| {
                    let mut chunk = chunk.into_iter();
                    let row1 = chunk.next().unwrap();
                    let row2 = chunk.next().unwrap_or(row1);  // if there is not 2nd row then the reflection is over
                    row1 == row2
                })
            {
                Some(1 + row_idx as u32)
            } else {
                None
            }
        }).collect::<Vec<_>>()
}

#[derive(Debug, PartialEq)]
enum Solution {
    None,
    Row(u32),
    Column(u32),
}

fn solve(lines: &[String]) -> (u32, u32) {
    let mut result_1 = 0;
    let mut result_2 = 0;

    lines.split(String::is_empty).for_each(|pattern| {
        // Solve Problem 1
        let solution = solve_map(pattern);
        assert!(solution.len() <= 1);  // First part of the problem assumes that there is only one reflection line

        let first_solution = if solution.len() == 1 {
            // Reflections accross a horizontal line are multiplied by 100
            result_1 += solution[0] * 100;
            Solution::Row(solution[0])
        } else {
            // Transpose and solve column-wise. Reflections accross a vertical line are not multiplied
            let transposed = transpose(pattern);
            let solution = solve_map(&transposed);
            assert!(solution.len() <= 1);
            if solution.len() == 1 {
                result_1 += solution[0];
                Solution::Column(solution[0])
            } else {
                Solution::None
            }
        };

        // Solve Problem 2
        // Fix the smudge withing the reflection
        let mut found = false;
        'outer: for row_idx in 0..pattern.len() {
            for column_idx in 0..pattern[0].len() {
                // Clone the pattern, flip character at (row_idx, column_idx) and solve
                let mut pattern = pattern.to_vec();
                let c = pattern[row_idx].chars().nth(column_idx).unwrap();
                pattern[row_idx].replace_range(column_idx..=column_idx, if c == '#' { "." } else { "#" });

                // We need to check solutions for both rows and columns since there may be multiple solutions
                let transposed = transpose(&pattern);
                let solutions_rows = solve_map(&pattern);
                let solutions_columns = solve_map(&transposed);
                if solutions_rows.is_empty() && solutions_columns.is_empty() {  // this flip yields no solutions
                    continue;
                }

                let solutions = solutions_rows
                    .into_iter()
                    .map(Solution::Row)
                    .chain(solutions_columns.into_iter().map(Solution::Column))
                    .collect::<Vec<_>>();

                let new_solution = solutions.iter().find(|&s| {
                    *s != first_solution
                });

                if let Some(new_solution) = new_solution {
                    match new_solution {
                        Solution::None => unreachable!(),
                        Solution::Row(rows_above) => {
                            result_2 += rows_above * 100;
                            found = true;
                        }
                        Solution::Column(columns_to_the_left) => {
                            result_2 += columns_to_the_left;
                            found = true;
                        }
                    }
                    break 'outer;
                }
            }
        }

        if !found {
            panic!("No smudge found");
        }
    });

    (result_1, result_2)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/13.txt").unwrap();

    let _example = [
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>();

    let (r1, r2) = solve(&_lines);
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
