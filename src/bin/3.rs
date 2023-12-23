use std::collections::HashMap;

use advent_of_code_2023::read_lines_as_vec;

// Returns tuple containing number and number of digits in the number
fn scan_number(input: &[u8]) -> Option<(u32, usize)> {
    let mut digits = 0;
    let mut number = 0;
    for char in input {
        if char.is_ascii_digit() {
            digits += 1;
            number = number * 10 + (char - b'0') as u32;
        } else {
            break;
        }
    }

    if digits == 0 {
        return None;
    }

    Some((number, digits))
}

// Scan if there's any symbol in the input
// Symbols are any character that is not a number or a dot
// If symbol is a gear (*) also return the index of the gear with respect to the input
fn scan_symbol(input: &[u8]) -> (bool, Option<usize>) {
    let mut symbol_found = false;
    let mut gear = None;
    for (index, char) in input.iter().enumerate() {
        if !char.is_ascii_digit() && char != &b'.' {
            symbol_found = true;
            if char == &b'*' {
                if gear.is_some() {
                    // Could be handled by returning a vector of gears, but it's extra work
                    // and it's not needed for the current problem
                    panic!("Number is adjacent to multiplce gears");
                }
                gear = Some(index);
            }
        }
    }

    (symbol_found, gear)
}

fn update_gears(
    gears: &mut HashMap<(u32, u32), (u32, u32)>,
    gear_row: u32,
    gear_column: u32,
    number: u32,
) {
    let gear_key = (gear_row, gear_column);
    gears
        .entry(gear_key)
        .and_modify(|(number_of_numbers, ratio)| {
            *ratio *= number;
            *number_of_numbers += 1;
        })
        .or_insert((1, number));
}

fn compute_results(input: &Vec<String>) -> (u32, u32) {
    let mut sum = 0;
    // (row, column) -> (number_of_adjacent_numbers, product_of_adjacent_numbers)
    let mut gears: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

    let n_lines = input.len();
    let n_columns = input[0].len();

    for i in 0..n_lines {
        let current_line = input[i].as_bytes();

        let mut j = 0;
        loop {
            if current_line[j] == b'.' {
                j += 1;
            } else if let Some((number, n_digits)) = scan_number(&current_line[j..]) {
                let left_symbol = if j > 0 {
                    // Scan symbol for 1 byte to the left
                    let (symbol, gear) = scan_symbol(&current_line[j - 1..j]);
                    if gear.is_some() {
                        update_gears(&mut gears, i as u32, (j - 1) as u32, number);
                    }
                    symbol
                } else {
                    false
                };

                let right_symbol = if j + n_digits < n_columns {
                    // Scan symbol for 1 byte to the right
                    let (symbol, gear) = scan_symbol(&current_line[j + n_digits..j + n_digits + 1]);
                    if gear.is_some() {
                        update_gears(&mut gears, i as u32, (j + n_digits) as u32, number);
                    }
                    symbol
                } else {
                    false
                };

                let top_symbol = if i > 0 {
                    // Scan symbol on top line including diagonals
                    let starting_column = if j > 0 { j - 1 } else { j };
                    let ending_column = if j + n_digits < n_columns {
                        j + n_digits + 1
                    } else {
                        j + n_digits
                    };
                    let (symbol, gear) =
                        scan_symbol(&input[i - 1].as_bytes()[starting_column..ending_column]);
                    if let Some(gear_idx) = gear {
                        update_gears(
                            &mut gears,
                            (i - 1) as u32,
                            (starting_column + gear_idx) as u32,
                            number,
                        );
                    }
                    symbol
                } else {
                    false
                };

                let bottom_symbol = if i + 1 < n_lines {
                    // Scan symbol on bottom line including diagonals
                    let starting_column = if j > 0 { j - 1 } else { j };
                    let ending_column = if j + n_digits < n_columns {
                        j + n_digits + 1
                    } else {
                        j + n_digits
                    };
                    let (symbol, gear) =
                        scan_symbol(&input[i + 1].as_bytes()[starting_column..ending_column]);
                    if let Some(gear_idx) = gear {
                        update_gears(
                            &mut gears,
                            (i + 1) as u32,
                            (starting_column + gear_idx) as u32,
                            number,
                        );
                    }
                    symbol
                } else {
                    false
                };

                let has_symbol = left_symbol || right_symbol || top_symbol || bottom_symbol;
                if has_symbol {
                    sum += number;
                }

                j += n_digits;
            } else {
                j += 1;
            }

            if j >= n_columns {
                break;
            }
        }
    }

    // Sum of ration in gears that have more than 1 adjacent numbers
    let gear_ratio = gears
        .iter()
        .filter(|(_, (n, _))| *n > 1)
        .map(|(_, (_, ratio))| ratio)
        .sum::<u32>();

    (sum, gear_ratio)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/3.txt").unwrap();

    let _example = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let (part_numbers, gear_ratio) = compute_results(&_lines);

    println!("Part numbers: {}", part_numbers);
    println!("Gear ratios: {}", gear_ratio);
}
