use std::collections::BTreeSet;

use advent_of_code_2023::read_lines_as_vec;
use itertools::Itertools;
use strum::IntoEnumIterator;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[derive(strum_macros::EnumIter)]
enum Cards {
    J = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Copy, Clone)]
struct Hand{
    // Five cards
    cards: [Cards; 5],
}

#[derive(Debug)]
struct Game {
    hand: (Hand, HandTypes),
    bid: u32,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum HandTypes {
    HighCard = 0,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand.1 != other.hand.1 {
            if self.hand.1 > other.hand.1 {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        } else {
            let self_cards = &self.hand.0.cards;
            let other_cards = &other.hand.0.cards;

            for (self_card, other_card) in self_cards.iter().zip(other_cards.iter()) {
                if self_card != other_card {
                    if self_card > other_card {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Less;
                    }
                }
            }

            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Game {}
impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

// Given a Hand, identify what is its type
fn identify_hand(hand: &Hand) -> HandTypes {
    let mut counts = [0u32; 13];

    // Count number of J
    let js = hand.cards.iter().filter(|&&card| card == Cards::J).count();
    if js > 0 {
        let mut highest_hand_type = HandTypes::HighCard;
        // Replace J with each of the other cards and call the function again
        for card in Cards::iter() {
            if card == Cards::J {
                continue;
            }

            let mut new_hand = *hand;
            new_hand.cards.iter_mut().for_each(|c| if *c == Cards::J { *c = card; });
            let hand_type = identify_hand(&new_hand);
            if hand_type > highest_hand_type {
                highest_hand_type = hand_type;
            }
        }

        return highest_hand_type;
    }

    // Count the occurrences of each label
    for card in &hand.cards {
        counts[*card as usize] += 1;
    }

    // Check for different hand types
    if counts.contains(&5) {
        HandTypes::FiveOfAKind
    } else if counts.contains(&4) {
        HandTypes::FourOfAKind
    } else if counts.contains(&3) && counts.contains(&2) {
        HandTypes::FullHouse
    } else if counts.contains(&3) {
        HandTypes::ThreeOfAKind
    } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
        HandTypes::TwoPairs
    } else if counts.contains(&2) {
        HandTypes::OnePair
    } else {
        HandTypes::HighCard
    }
}

fn calculate_result(input: &Vec<String>) -> u32 {
    let mut games = BTreeSet::new();

    for line in input {
        let (hand, bid) = line.split_whitespace().take(2).collect_tuple().unwrap();
        let bid = bid.parse::<u32>().unwrap();

        // Iterate over the 5 characters of the hand and convert them to Cards
        let hand = hand.chars().map(|c| match c {
            'J' => Cards::J,
            '2' => Cards::Two,
            '3' => Cards::Three,
            '4' => Cards::Four,
            '5' => Cards::Five,
            '6' => Cards::Six,
            '7' => Cards::Seven,
            '8' => Cards::Eight,
            '9' => Cards::Nine,
            'T' => Cards::T,
            'Q' => Cards::Q,
            'K' => Cards::K,
            'A' => Cards::A,
            _ => panic!("Invalid card"),
        }).collect::<Vec<_>>();

        let hand = Hand {
            cards: [hand[0], hand[1], hand[2], hand[3], hand[4]],
        };

        let hand_type = identify_hand(&hand);
        let game = Game { hand: (hand, hand_type), bid };
        games.insert(game);
    }

    // Iterate over ordered games and calculate winnings
    let mut winnings = 0;
    for (rank, game) in games.iter().enumerate() {
        winnings += game.bid * (rank + 1) as u32;
    }

    winnings
}

fn main() {
    let _input = read_lines_as_vec("inputs/7.txt").unwrap();

    let _example = vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string(),
    ];
    let winnings = calculate_result(&_input);
    println!("Winnings: {}", winnings);
}
