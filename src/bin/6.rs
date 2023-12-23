use advent_of_code_2023::read_lines_as_vec;

struct Race {
    time: u64,
    max_distance: u64,
}

fn parse(input: &[String]) -> Vec<Race> {
    // Parse first line - Times
    let times = input[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Parse max distances
    let max_distances = input[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Take both and transform into Race
    times
        .iter()
        .zip(max_distances.iter())
        .map(|pair| Race {
            time: *pair.0,
            max_distance: *pair.1,
        })
        .collect()
}

fn parse_part2(input: &[String]) -> Vec<Race> {
    let time = input[0].split(':').nth(1).unwrap()
    .replace(' ', "").parse::<u64>().unwrap();

    let max_distance = input[1].split(':').nth(1).unwrap()
    .replace(' ', "").parse::<u64>().unwrap();

    vec![
        Race {
            time,
            max_distance,
        }
    ]
}

fn calculate_result(races: &Vec<Race>) -> u64 {
    let mut product = 1;

    for race in races {
        let t = race.time as f64;
        let d = race.max_distance as f64;

        // t = (T +- sqrt(T² - 4D))/2
        let t1 = (t + (t * t - 4.0 * d).sqrt()) / 2.0;
        let t2 = (t - (t * t - 4.0 * d).sqrt()) / 2.0;

        let t1 = if t1.fract().abs() > 0.0 {
            t1.floor() as u64
        } else {
            t1.floor() as u64 - 1
        };

        let t2 = if t2.fract().abs() > 0.0 {
            t2.ceil() as u64
        } else {
            t2.ceil() as u64 + 1
        };

        let number_of_ways = t1 - t2 + 1;
        product *= number_of_ways;
    }

    product
}

fn main() {
    let _lines = read_lines_as_vec("inputs/6.txt").unwrap();

    let _example = vec![
        "Time:      7  15   30".to_owned(),
        "Distance:  9  40  200".to_owned(),
    ];

    let races = parse(&_lines);
    let races_part_2 = parse_part2(&_lines);

    let result = calculate_result(&races);
    let result_part_2 = calculate_result(&races_part_2);

    println!("Result: {}", result);
    println!("Result part 2: {}", result_part_2);
}
