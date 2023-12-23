use std::collections::HashMap;

use advent_of_code_2023::read_lines;

fn calculate_sum<'a, T: Iterator<Item = &'a String>>(lines: T) -> u32 {
    let mut sum = 0;
    for line in lines {
        // first digit
        for character in line.chars() {
            if character.is_ascii_digit() {
                sum += character.to_digit(10).unwrap() * 10;
                break;
            }
        }

        // last digit
        for character in line.chars().rev() {
            if character.is_ascii_digit() {
                sum += character.to_digit(10).unwrap();
                break;
            }
        }
    }

    sum
}

fn calculate_sum_with_words<'a, T: Iterator<Item = &'a String>>(lines: T) -> u32 {
    let map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;
    for line in lines {
        let mut first_index = std::usize::MAX;
        let mut last_index = 0;
        let mut first_digit = 0;
        let mut last_digit = 0;

        for digit in map.keys() {
            for (index, _) in line.match_indices(digit) {
                if index < first_index {
                    first_index = index;
                    first_digit = map[digit];
                }
                if index >= last_index {
                    last_index = index;
                    last_digit = map[digit];
                }
            }
        }

        sum += first_digit * 10 + last_digit;
        // println!("{line} = {} {}", first_digit, last_digit)
    }

    sum
}

fn main() {
    let lines = read_lines("inputs/1.txt").unwrap();

    let _lines: Vec<String> = lines.flatten().collect();

    let sum = calculate_sum(_lines.iter());

    let _example = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
        "1oneightq".to_string(),
        "4kok".to_string(),
        "twor934onetwo".to_string(),
    ];

    let sum_words = calculate_sum_with_words(_lines.iter());

    println!("{}", sum);
    println!("{}", sum_words);
}
