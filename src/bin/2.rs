use advent_of_code_2023::read_lines;

#[derive(Debug)]
struct Draw {
    blue: u32,
    red: u32,
    green: u32,
}

fn compute_result<'a, T: Iterator<Item = &'a String>>(lines: T) -> (u32, u32) {
    let regex_game_id = regex::Regex::new(r#"Game (\d+)"#).unwrap();
    let regex_cubes = regex::Regex::new(r#"(?<num>\d+) (?<color>\w+)"#).unwrap();
    let mut sum = 0;
    let mut sum_of_powers = 0;

    for line in lines {
        let game_id = regex_game_id.captures(line).unwrap().get(1).unwrap().as_str();
        let game_id = game_id.parse::<u32>().unwrap();

        let mut draws = Vec::new();
        let x: Vec<&str> = line.split(':').skip(1).flat_map(|s| s.split(';')).collect();
        for item in x {
            let mut draw = Draw {
                blue: 0,
                red: 0,
                green: 0,
            };

            for capture in regex_cubes.captures_iter(item) {
                match &capture["color"] {
                    "blue" => draw.blue += capture["num"].parse::<u32>().unwrap(),
                    "red" => draw.red += capture["num"].parse::<u32>().unwrap(),
                    "green" => draw.green += capture["num"].parse::<u32>().unwrap(),
                    _ => panic!("Unknown color"),
                }
            }

            draws.push(draw);
        }

        // Check if draw is possible based on the limits
        let failing_draws = draws.iter().filter(|&d| d.red > 12 || d.green > 13 || d.blue > 14).count();
        if failing_draws == 0 {
            sum += game_id;
        }

        // Iterate over draws to create another draw comprised on the largest values for each color
        let mut minimum_draw = Draw {
            blue: 0,
            red: 0,
            green: 0,
        };
        draws.iter().for_each(|d| {
            minimum_draw.blue = minimum_draw.blue.max(d.blue);
            minimum_draw.red = minimum_draw.red.max(d.red);
            minimum_draw.green = minimum_draw.green.max(d.green);
        });

        let power = minimum_draw.blue * minimum_draw.red * minimum_draw.green;
        sum_of_powers += power;
    }

    (sum, sum_of_powers)
}

fn main() {
    let lines = read_lines("inputs/2.txt").unwrap();

    // let data = vec![
    //     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    //     "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    //     "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    //     "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    //     "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    // ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();


    let (sum_of_ids, sum_of_powers) = compute_result(lines.flatten().collect::<Vec<String>>().iter());
    println!("Sum of ids: {}", sum_of_ids);
    println!("Sum of powers: {}", sum_of_powers);
}
