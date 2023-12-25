use advent_of_code_2023::read_lines_as_vec;
use itertools::Itertools;

// Part 1 and 2 - Process the sequence to obtain the next number
fn process_sequence(sequence: &[i64], backwards: bool) -> i64 {
    // Stop condition is all elements are zero
    if sequence.iter().all(|&n| n == 0) {
        return 0;
    }

    // Calculate difference between each number
    let differences = sequence.iter().tuple_windows().map(|(a, b)| b-a).collect::<Vec<_>>();

    // Are we calculating the next number or the number before the first
    if backwards {
        return sequence.first().unwrap() - process_sequence(&differences, true);
    } else {
        return sequence.last().unwrap() + process_sequence(&differences, false);
    }
}

fn calculate_result(lines: &[String]) -> (i64, i64) {
    let mut sum_of_next_numbers = 0;
    let mut sum_of_next_numbers_backwards = 0;
    for line in lines {
        let starting_sequence = line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let next_number = process_sequence(&starting_sequence, false);
        let next_number_backwards = process_sequence(&starting_sequence, true);
        sum_of_next_numbers += next_number;
        sum_of_next_numbers_backwards += next_number_backwards;
    }

    (sum_of_next_numbers, sum_of_next_numbers_backwards)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/9.txt").unwrap();

    let _example = r#"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "#.lines().map(|s| s.to_string()).collect::<Vec<_>>();

    let (result1, result2) = calculate_result(&_lines);
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}
