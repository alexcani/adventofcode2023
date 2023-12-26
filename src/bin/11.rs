use advent_of_code_2023::read_lines_as_vec;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Element {
    Empty,
    Galaxy
}

fn calculate_result(lines: &[String]) -> (u32, u64) {
    let rows = lines.len();
    let cols = lines[0].len();

    let mut map = vec![Vec::with_capacity(cols); rows];
    let mut galaxies = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => map[i].push(Element::Empty),
                '#' => {
                    map[i].push(Element::Galaxy);
                    galaxies.push((i as u32, j as u32));
                },
                _ => panic!("Unknown element: {}", c)
            }
        }
    }

    // Rows and columns that consist only of empty space need to be duplicated
    // Collect the indices of these rows and columns for later
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    for (i, row) in map.iter().enumerate() {
        if row.iter().all(|e| *e == Element::Empty) {
            empty_rows.push(i);
        }
    }
    for j in 0..cols {
        if map.iter().all(|row| row[j] == Element::Empty) {
            empty_cols.push(j);
        }
    }

    // For every pair of galaxies
    let mut sum_of_shortest_distances_p1 = 0;
    let mut sum_of_shortest_distances_p2: u64 = 0;
    for (i, first) in galaxies.iter().enumerate() {  // moves sequentially from galaxy x to n
        for second in galaxies.iter().skip(i + 1) {  // from galaxy x+1 onwards, guarantees unique pairs
            // Initial distance between them
            sum_of_shortest_distances_p1 += second.0.abs_diff(first.0) + second.1.abs_diff(first.1);
            sum_of_shortest_distances_p2 += (second.0.abs_diff(first.0) + second.1.abs_diff(first.1)) as u64;

            // For every empty row or column between the two galaxies increase the distance by 1
            for row in empty_rows.iter() {
                if *row as u32 > first.0.min(second.0) && (*row as u32) < second.0.max(first.0) {
                    sum_of_shortest_distances_p1 += 1;
                    sum_of_shortest_distances_p2 += 1000000 - 1;
                }
            }
            for col in empty_cols.iter() {
                if *col as u32 > first.1.min(second.1) && (*col as u32) < second.1.max(first.1) {
                    sum_of_shortest_distances_p1 += 1;
                    sum_of_shortest_distances_p2 += 1000000  - 1;
                }
            }
        }
    }

    (sum_of_shortest_distances_p1, sum_of_shortest_distances_p2)
}

fn main() {
    let _lines = read_lines_as_vec("inputs/11.txt").unwrap();

    let _example = [
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ].iter().map(|s| s.to_string()).collect::<Vec<_>>();

    let (sum_of_shortest_distances_p1, sum_of_shortest_distances_p2) = calculate_result(&_lines);
    println!("Part 1: {}", sum_of_shortest_distances_p1);
    println!("Part 2: {}", sum_of_shortest_distances_p2);
}
