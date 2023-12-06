use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
}

#[derive(Debug, Copy, Clone)]
struct Map {
    source: usize,
    destination: usize,
    length: usize,
}

impl Map {
    fn new(destination: usize, source: usize, length: usize) -> Self {
        Self {
            source,
            destination,
            length,
        }
    }

    fn partner(&self, source: usize) -> Option<usize> {
        if source >= self.source && source < self.source + self.length {
            Some(self.destination + (source - self.source))
        } else {
            None
        }
    }
}

fn part2(input: &str) -> usize {
    let (seed_section, sections) = input.split_once("\n\n").expect("failed to split input");
    let seed_list: Vec<_> = seed_section
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    let seeds = seed_list
        .chunks_exact(2)
        .flat_map(|c| (c[0]..(c[0] + c[1])))
        .collect();
    solve(seeds, sections)
}

fn part1(input: &str) -> usize {
    let (seed_section, sections) = input.split_once("\n\n").expect("failed to split input");
    let seeds: Vec<_> = seed_section
        .split(" ")
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();
    solve(seeds, sections)
}

fn solve(seeds: Vec<usize>, sections: &str) -> usize {
    let map_collection: Vec<_> = sections
        .split("\n\n")
        .map(|s| {
            s.lines()
                .enumerate()
                .filter_map(|(i, l)| {
                    if i == 0 {
                        return None;
                    }
                    let mut parts = l.split(" ");
                    let source = parts.next()?.parse::<usize>().ok()?;
                    let destination = parts.next()?.parse::<usize>().ok()?;
                    let length = parts.next()?.parse::<usize>().ok()?;
                    Some(Map::new(source, destination, length))
                })
                .collect::<Vec<_>>()
        })
        .collect();

    seeds
        .par_iter()
        .map(|s| {
            let mut current = *s;
            for map_group in map_collection.iter() {
                let mut next = current;
                for map in map_group.iter() {
                    if let Some(partner) = map.partner(current) {
                        next = partner;
                        break;
                    }
                }
                current = next;
            }
            current
        })
        .min()
        .expect("should have min item")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input");
        let result = part1(&input);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_map() {
        let map = Map::new(52, 50, 48);
        assert_eq!(map.partner(79), Some(81));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input");
        let result = part2(&input);
        assert_eq!(result, 46);
    }
}
