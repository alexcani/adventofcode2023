use advent_of_code_2023::read_lines_as_vec;

#[derive(Debug)]
struct Map {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl FromIterator<u64> for Map {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let destination_start = iter.next().unwrap();
        let source_start = iter.next().unwrap();
        let length = iter.next().unwrap();

        Map {
            destination_start,
            source_start,
            length,
        }
    }
}

// Applies a mapping to an input
fn use_map(input: u64, map: &Vec<Map>) -> u64 {
    // Iterates over all maps and check if input falls within the range
    for m in map {
        if input >= m.source_start && input < m.source_start + m.length {
            return input - m.source_start + m.destination_start;
        }
    }

    // If no map is found, return the input
    input
}

#[derive(Debug)]
enum NextStep {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

fn calculate_result(input: &Vec<String>) -> u64 {
    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    // Parse seeds in first line (Problem 1)
    let seeds: Vec<u64> = input[0].split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();

    // Parse the rest of the lines, first step is seed so that it skips first line
    let mut next_step = NextStep::Seed;
    for line in input {
        if line.is_empty() {
            continue;
        } else if line == "seed-to-soil map:" {
            next_step = NextStep::Soil;
            continue;
        } else if line == "soil-to-fertilizer map:" {
            next_step = NextStep::Fertilizer;
            continue;
        } else if line == "fertilizer-to-water map:" {
            next_step = NextStep::Water;
            continue;
        } else if line == "water-to-light map:" {
            next_step = NextStep::Light;
            continue;
        } else if line == "light-to-temperature map:" {
            next_step = NextStep::Temperature;
            continue;
        } else if line == "temperature-to-humidity map:" {
            next_step = NextStep::Humidity;
            continue;
        } else if line == "humidity-to-location map:" {
            next_step = NextStep::Location;
            continue;
        }

        let values: Map = line.split_whitespace().filter_map(|s| s.parse::<u64>().ok()).take(3).collect();
        match next_step {
            NextStep::Seed => (),
            NextStep::Soil => seed_to_soil.push(values),
            NextStep::Fertilizer => soil_to_fertilizer.push(values),
            NextStep::Water => fertilizer_to_water.push(values),
            NextStep::Light => water_to_light.push(values),
            NextStep::Temperature => light_to_temperature.push(values),
            NextStep::Humidity => temperature_to_humidity.push(values),
            NextStep::Location => humidity_to_location.push(values),
        }
    }

    // Iterate over seeds and apply all maps
    let mut lowest_location = u64::MAX;
    for seed in seeds {
        let soil = use_map(seed, &seed_to_soil);
        let fertilizer = use_map(soil, &soil_to_fertilizer);
        let water = use_map(fertilizer, &fertilizer_to_water);
        let light = use_map(water, &water_to_light);
        let temperature = use_map(light, &light_to_temperature);
        let humidity = use_map(temperature, &temperature_to_humidity);
        let location = use_map(humidity, &humidity_to_location);

        if location < lowest_location {
            lowest_location = location;
        }
    }

    lowest_location
}

fn main() {
    let _input = read_lines_as_vec("inputs/5.txt").unwrap();

    let _example = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ].into_iter().map(|s| s.to_owned()).collect::<Vec<String>>();

    let lowest_location = calculate_result(&_input);
    println!("Lowest location: {:?}", lowest_location);

}
