use advent_of_code_2023::read_lines_as_vec;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pulse {
    Low,
    High,
}

trait ModuleFunction {
    fn process_input(&mut self, pulse: &Pulse, source: &str) -> Option<Pulse>;
    fn add_input(&mut self) {}
}

struct FlipFlop {
    on: bool,
}

impl FlipFlop {
    fn new() -> Self {
        FlipFlop { on: false }
    }
}

impl ModuleFunction for FlipFlop {
    fn process_input(&mut self, pulse: &Pulse, _: &str) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.on = !self.on;
                Some(if self.on { Pulse::High } else { Pulse::Low })
            }
        }
    }
}

struct Conjunction {
    next_input: usize,
    source_input_map: HashMap<String, usize>,
    inputs: Vec<Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Conjunction {
            source_input_map: HashMap::new(),
            inputs: Vec::new(),
            next_input: 0,
        }
    }
}

impl ModuleFunction for Conjunction {
    fn add_input(&mut self) {
        self.inputs.push(Pulse::Low); // initial state is Low for input ports
    }

    fn process_input(&mut self, pulse: &Pulse, source: &str) -> Option<Pulse> {
        let input_index = self
            .source_input_map
            .get(source)
            .cloned()
            .unwrap_or_else(|| {
                // if we haven't seen this source before, allocate a new input port and add to map
                self.source_input_map
                    .insert(source.to_string(), self.next_input);
                self.next_input += 1;
                self.next_input - 1
            });

        self.inputs[input_index] = *pulse;

        if self.inputs.iter().all(|pulse| *pulse == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

struct Broadcaster {}
impl Broadcaster {
    fn new() -> Self {
        Broadcaster {}
    }
}
impl ModuleFunction for Broadcaster {
    fn process_input(&mut self, pulse: &Pulse, _: &str) -> Option<Pulse> {
        Some(*pulse) // broadcast input to all outputs
    }
}

struct Button {}
impl Button {
    fn new() -> Self {
        Button {}
    }
}
impl ModuleFunction for Button {
    fn process_input(&mut self, _: &Pulse, _: &str) -> Option<Pulse> {
        Some(Pulse::Low)
    }
}

struct Event {
    origin: String,
    destination: String,
    pulse: Pulse,
}

struct Module {
    name: String,
    n_low_pulses: u64,
    n_high_pulses: u64,
    destinations: Vec<String>,
    function: Box<dyn ModuleFunction>,
}

impl Module {
    fn new(name: String, destinations: Vec<String>, function: Box<dyn ModuleFunction>) -> Self {
        Module {
            name,
            n_low_pulses: 0,
            n_high_pulses: 0,
            destinations,
            function,
        }
    }

    fn process_input(&mut self, event: &Event) -> Option<Vec<Event>> {
        let outward_pulse = self.function.process_input(&event.pulse, &event.origin);
        if let Some(pulse) = outward_pulse {
            match pulse {
                Pulse::Low => self.n_low_pulses += self.destinations.len() as u64,
                Pulse::High => self.n_high_pulses += self.destinations.len() as u64,
            }

            return Some(
                self.destinations
                    .iter()
                    .map(|destination| Event {
                        origin: self.name.clone(),
                        destination: destination.clone(),
                        pulse,
                    })
                    .collect::<Vec<Event>>(),
            );
        }

        None
    }

    fn low_pulses(&self) -> u64 {
        self.n_low_pulses
    }

    fn high_pulses(&self) -> u64 {
        self.n_high_pulses
    }

    fn add_input(&mut self) {
        self.function.add_input();
    }
}

fn parse(input: &[String]) -> HashMap<String, Module> {
    let mut modules = HashMap::new();

    for line in input {
        let mut parts = line.trim().split(" -> ");
        let function = parts.next().unwrap();
        let destinations = parts
            .next()
            .unwrap()
            .split(',')
            .map(|d| d.trim().to_owned())
            .collect::<Vec<_>>();

        if function == "broadcaster" {
            // special name
            let module = Module::new(
                "broadcaster".to_string(),
                destinations,
                Box::new(Broadcaster::new()),
            );
            modules.insert("broadcaster".to_owned(), module);
            continue;
        }

        let mod_type = &function[0..1];
        let mod_name = &function[1..];

        let module = match mod_type {
            "%" => Module::new(
                mod_name.to_string(),
                destinations,
                Box::new(FlipFlop::new()),
            ),
            "&" => Module::new(
                mod_name.to_string(),
                destinations,
                Box::new(Conjunction::new()),
            ),
            _ => panic!("Unknown module type: {}", mod_type),
        };

        modules.insert(mod_name.to_owned(), module);
    }

    // Add button
    let button = Module::new(
        "button".to_string(),
        vec!["broadcaster".to_string()],
        Box::new(Button::new()),
    );
    modules.insert("button".to_owned(), button);

    // Go through each module to add inputs, clone keys to avoid borrowing issues
    let keys = modules.keys().cloned().collect::<Vec<_>>();

    for key in keys {
        let destinations = modules.get(&key).unwrap().destinations.clone();
        for destination in destinations {
            modules
                .entry(destination)
                .and_modify(|module| module.add_input());  // may not exist since some destinations are untyped
        }
    }

    modules
}

fn solve(mut modules: HashMap<String, Module>, n_button_presses: u64, stop_on_rx: bool) -> u64 {
    let mut event_queue = VecDeque::new();

    let mut button_presses = 1;
    loop {
        if stop_on_rx && button_presses % 100000 == 0 {
            println!("button press {}", button_presses);
        }

        // Push button press event as first event
        event_queue.push_back(Event {
            origin: "god".to_string(),
            destination: "button".to_string(),
            pulse: Pulse::Low, // for the button it doesn't matter
        });

        while let Some(event) = event_queue.pop_front() {
            if event.destination == "ll" && event.pulse == Pulse::High {
                // Only source for "rx" is "ll", which is a conjunction.
                // "ll" needs to receive high pulse from all it's inputs to send a low pulse to "rx"
                // Manually check for this condition and return the number of button presses for each of "ll"'s inputs
                // to send it a high pulse.
                // Since it has 4 inputs, get the lcm of the number of button presses for each input and multiply them
                // to obtain the number of button presses where all inputs simultaneously send a high pulse.
                // hacky but works
                println!("ll received high pulse at button press {} from {}", button_presses, event.origin);
            }

            if stop_on_rx && event.destination == "rx" && event.pulse == Pulse::Low {
                println!("rx received low pulse at button press {}", button_presses);
                return button_presses;
            }

            let module = modules.get_mut(&event.destination);
            if module.is_none() {
                // destination is untyped, ignore
                continue;
            }
            if let Some(events) = module.unwrap().process_input(&event) {
                event_queue.extend(events);
            }
        }

        button_presses += 1;
        if stop_on_rx || button_presses <= n_button_presses {
            continue;
        }
        break;
    }

    let mut total_low_pulses = 0;
    let mut total_high_pulses = 0;
    for module in modules.values() {
        total_low_pulses += module.low_pulses();
        total_high_pulses += module.high_pulses();
    }

    total_low_pulses * total_high_pulses
}

fn main() {
    let _lines = read_lines_as_vec("inputs/20.txt").unwrap();
    let _example1 = r#"broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a"#
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let _example2 = r#"broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output"#
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let result = solve(parse(&_lines), 1000, false);
    println!("Result: {}", result);

    // Solve part 2 - find the number of button presses that will cause rx to receive a low pulse
    solve(parse(&_lines), 0, true);
}
