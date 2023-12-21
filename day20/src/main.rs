#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    FlipFlop {
        name: String,
        on: bool,
        destinations: Vec<String>,
    },
    Conjunction {
        name: String,
        memory: HashMap<String, Pulse>,
        destinations: Vec<String>,
    },
    Broadcaster {
        name: String,
        destinations: Vec<String>,
    },
}

#[derive(Debug)]
struct State {
    modules: HashMap<String, Module>,
    queue: VecDeque<(String, Pulse, String)>,
    pulse_count: HashMap<Pulse, usize>,
}

impl State {
    fn new(modules: HashMap<String, Module>) -> Self {
        let mut pulse_count = HashMap::new();
        pulse_count.insert(Pulse::Low, 0);
        pulse_count.insert(Pulse::High, 0);
        Self {
            modules,
            queue: VecDeque::new(),
            pulse_count,
        }
    }

    fn process_queue(&mut self) {
        while let Some((producer, pulse, target)) = self.queue.pop_front() {
            self.pulse_count.entry(pulse).and_modify(|c| *c += 1);

            match self.modules.get_mut(&target) {
                Some(Module::FlipFlop {
                    on,
                    destinations,
                    name,
                }) => {
                    match pulse {
                        Pulse::Low => {
                            *on = !*on;
                            for dest in destinations {
                                if *on {
                                    self.queue.push_back((
                                        name.to_string(),
                                        Pulse::High,
                                        dest.clone(),
                                    ));
                                } else {
                                    self.queue.push_back((
                                        name.to_string(),
                                        Pulse::Low,
                                        dest.clone(),
                                    ));
                                }
                            }
                        }
                        Pulse::High => (),
                    }
                    //
                }
                Some(Module::Conjunction {
                    memory,
                    destinations,
                    name,
                }) => {
                    memory.insert(producer, pulse);
                    let pulse = if memory.values().all(|p| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for dest in destinations {
                        self.queue
                            .push_back((name.to_string(), pulse, dest.clone()));
                    }
                }
                Some(Module::Broadcaster { destinations, name }) => {
                    for dest in destinations {
                        self.queue
                            .push_back((name.to_string(), pulse, dest.clone()));
                    }
                }
                // skip unnamed modules
                None => (),
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);
}

fn part1(input: &str) -> usize {
    let mut targeting = HashMap::new();
    let mut modules: HashMap<_, _> = input
        .lines()
        .filter_map(|l| {
            let (name, dests) = l.split_once(" -> ").expect("failed to split line");
            let dests: Vec<_> = dests.split(", ").map(|s| s.to_string()).collect();
            for dest in dests.clone() {
                targeting
                    .entry(dest)
                    .or_insert_with(Vec::new)
                    .push(name[1..].to_string());
            }
            let (name, module) = if name.starts_with('%') {
                (
                    &name[1..],
                    Module::FlipFlop {
                        name: name[1..].to_string(),
                        on: false,
                        destinations: dests,
                    },
                )
            } else if name.starts_with('&') {
                (
                    &name[1..],
                    Module::Conjunction {
                        name: name[1..].to_string(),
                        memory: HashMap::new(),
                        destinations: dests,
                    },
                )
            } else {
                (
                    name,
                    Module::Broadcaster {
                        name: name.to_string(),
                        destinations: dests,
                    },
                )
            };
            Some((name.to_owned(), module))
        })
        .collect();

    for (name, module) in modules.iter_mut() {
        match module {
            Module::FlipFlop { .. } | Module::Broadcaster { .. } => (),
            Module::Conjunction { memory, .. } => {
                for t in targeting.get(name).expect("failed to get targeting") {
                    memory.insert(t.clone(), Pulse::Low);
                }
            }
        }
    }
    let mut state = State::new(modules);
    for _ in 0..1000 {
        state
            .queue
            .push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        state.process_queue();
    }
    let high_count = *state
        .pulse_count
        .get(&Pulse::High)
        .expect("failed to get pulse count");
    let low_count = *state
        .pulse_count
        .get(&Pulse::Low)
        .expect("failed to get pulse count");
    high_count * low_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 32000000);

        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(part1(input), 11687500);
    }
}
