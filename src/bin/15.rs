use std::collections::VecDeque;

use advent_of_code_2023::read_lines_as_vec;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    power: u8,
}

fn hash(input: &str) -> u8 {
    let mut result = 0;
    input.chars().for_each(|c| {
        result += c as u16;
        result *= 17;
        result %= 256;
    });
    result as u8
}

fn solve(input: &str) -> (u32, u32) {
    // Part 1 - Sum of all hashes
    let r1 = input.split(',').map(|step| {
        hash(step) as u32
    }).collect::<Vec<_>>().iter().sum::<u32>();

    // Part 2 - Boxes
    let mut boxes: Vec<VecDeque<Lens>> = vec![VecDeque::new(); 256];
    input.split(',').for_each(|step| {
        let mut parts = step.split(&['=', '-']);
        let label = parts.next().unwrap();
        let power = parts.next().unwrap();
        let hash = hash(label) as usize;
        let index = boxes[hash].iter().position(|l| l.label == label);

        if !power.is_empty() {  // Put or replace lens in box
            let power = power.parse::<u8>().unwrap();
            let lens = Lens { label: label.to_string(), power };
            if let Some(index) =  index {
                boxes[hash].remove(index);  // Remove old lens and insert new in the same place
                boxes[hash].insert(index, lens);
            } else {
                boxes[hash].push_back(lens);
            }
        } else {  // Remove lens from box
            if let Some(index) = index {
                boxes[hash].remove(index);
            }
        }
    });

    // Calculate result
    let mut r2 = 0;
    boxes.iter().enumerate().for_each(|(i, box_)| {
        box_.iter().enumerate().for_each(|(j, lens)| {
            r2 += (1+i as u32) * (1+j as u32) * (lens.power as u32);
        });
    });

    (r1, r2)
}

fn main() {
    let _lines = &read_lines_as_vec("inputs/15.txt").unwrap()[0];
    let _example = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    // Input is a single line
    let (r1, r2) = solve(_lines);
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}
