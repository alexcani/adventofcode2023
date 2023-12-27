use advent_of_code_2023::read_lines_as_vec;
use std::collections::HashMap;
use itertools::{Itertools};

// Springs is the string of operational, defective and unkown springs
// Counts is the sequence of groups of broken springs
// Chunk is the number of broken springs in the group we're currently looking at
fn solve(springs: &[char], counts: &[u32], chunk: u32, cache: &mut HashMap<(usize, usize, usize), u64>) -> u64 {
    if springs.is_empty() {
        return match (chunk, counts.len()) {
            (0, 0) => 1,  // end of string and end of counts, count possibility
            (chunk, 1) if chunk == counts[0] => 1, // finished string with a # and it's the last one we needed to make the last count
            _ => 0  // everything else doesn't result in a possible combination
        }
    }

    // Not the end of the string, but we're out of counts and found a #
    if chunk > 0 && counts.is_empty() {
        return 0;
    }

    // Check if we've already calculated this combination
    let k = (springs.len(), chunk as usize, counts.len());
    if let Some(&result) = cache.get(&k) {
        return result;
    }

    // Process the next character
    let arrangements = match (&springs[0], chunk) {
        ('.', 0) => solve(&springs[1..], counts, 0, cache),  // not processing a chunk, found .
        ('.', chunk) if chunk != counts[0] => 0,  // found . but chunk isn't yet complete
        ('.', _) => solve(&springs[1..], &counts[1..], 0, cache),  // found . and it completes the chunk, get next count
        ('#', chunk) => solve(&springs[1..], counts, chunk+1, cache),  // found #, increase chunk (start processing or continue)
        ('?', 0) => {  // found ? and not computing any chunk
            let mut ways = solve(&springs[1..], counts, 1, cache);  // count as a #, start computing chunk
            ways += solve(&springs[1..], counts, 0, cache);  // count as a .
            ways
        }
        ('?', chunk) => {  // found ? and computing chunk
            let mut ways = solve(&springs[1..], counts, chunk+1, cache);  // count as a #
            if chunk == counts[0] {  // counting as . will complete the chunk
                ways += solve(&springs[1..], &counts[1..], 0, cache);  // get next count
            }  // else case would always result in failure, so don't bother
            ways
        },
        _ => unreachable!()
    };
    cache.insert(k, arrangements);
    arrangements
}

fn calculate_result(input: &[String]) -> (u64, u64) {
    input.iter().map(|line| {
        let mut cache = HashMap::new();

        let (springs, counts) = line.split_whitespace().collect_tuple().unwrap();

        // Part 1
        let counts = counts.split(',').map(|s| s.parse::<u32>().unwrap()).collect_vec();
        let p1 = solve(&springs.chars().collect::<Vec<_>>(), &counts, 0, &mut cache);

        // Part 2 - Unfold springs and counts 5 times
        let counts = counts.repeat(5);
        let springs = std::iter::once(springs).cycle().take(5).join("?");
        let p2 = solve(&springs.chars().collect::<Vec<_>>(), &counts, 0, &mut cache);
        (p1, p2)
    }).reduce(|(p1, p2), elem| (p1 + elem.0, p2 + elem.1)).unwrap()
}

fn main() {
    let _lines = read_lines_as_vec("inputs/12.txt").unwrap();

    let _example = [
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let (p1, p2) = calculate_result(&_lines);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
