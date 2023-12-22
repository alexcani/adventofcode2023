use std::collections::{HashSet, HashMap};

use advent_of_code_2023::read_lines_as_vec;

fn calculate_result(input: &Vec<String>) -> (u32, u32) {
    let mut sum = 0;
    let mut card_number = 1;
    // Card number to number of cards with this number
    let mut cards_map: HashMap<u32, u32> = HashMap::new();

    for line in input {
        // First appearance of card or increase the number of cards
        // Since by the time the original card is processed (from the strings) there is no way to increase the number of this card,
        // we can hold on to the final number of this card
        let number_of_current_cards_of_this_card = *cards_map
            .entry(card_number)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        let game_and_numbers = line.split(':').collect::<Vec<&str>>();
        let winners_and_our_numbers = game_and_numbers[1].split('|').collect::<Vec<&str>>();

        let lucky_numbers: HashSet<u32> = winners_and_our_numbers[0].split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect();
        let my_numbers: HashSet<u32> = winners_and_our_numbers[1].split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect();

        let numbers_in_both = lucky_numbers.intersection(&my_numbers).count();
        let points = if numbers_in_both > 0 {
            2u32.pow((numbers_in_both - 1) as u32)
        } else {
            0
        };

        sum += points; // Points for the first problem don't consider the cloned cards

        // Add N cloned cards for the `numbers_in_both` subsequent cards, where N is the number of cards of the current card
        for i in 1..=numbers_in_both {
            cards_map
            .entry(card_number+i as u32) // take the i-th next card
            .and_modify(|x| *x += number_of_current_cards_of_this_card) // for each card of the current card, increase the number of cards of the i-th next card
            .or_insert(number_of_current_cards_of_this_card);  // at least this amount of cards of the i-th next card
        }

        // Rely on the fact that cards come sequentially. Alternative would be parse from the string
        card_number += 1;
    }

    let total_number_of_cards = cards_map.values().sum::<u32>();

    (sum, total_number_of_cards)
}

fn main() {
    let _input = read_lines_as_vec("inputs/4.txt").unwrap();

    let _example = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ].into_iter().map(|s| s.to_owned()).collect::<Vec<String>>();

    let (sum_of_points, total_cards) = calculate_result(&_input);
    println!("Sum of points: {}", sum_of_points);
    println!("Total cards: {}", total_cards);
}
