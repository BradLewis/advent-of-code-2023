#![allow(dead_code)]

use std::{collections::HashSet, fs, ops::Index};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Ground {
    Garden,
    Rock,
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Garden,
            '#' => Self::Rock,
            _ => panic!("invalid ground type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Map {
    ground: Vec<Vec<Ground>>,
    width: usize,
    height: usize,
    start_position: Position,
}

impl Map {
    fn print(&self, positions: &HashSet<Position>) {
        for (y, line) in self.ground.iter().enumerate() {
            for (x, ground) in line.iter().enumerate() {
                let position = Position::new(x, y);
                if positions.contains(&position) {
                    print!("O");
                } else {
                    match ground {
                        Ground::Garden => print!("."),
                        Ground::Rock => print!("#"),
                    }
                }
            }
            println!();
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut start_position = Position::new(0, 0);
        let map: Vec<Vec<_>> = value
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start_position = Position::new(x, y);
                            Ground::from('.')
                        } else {
                            Ground::from(c)
                        }
                    })
                    .collect()
            })
            .collect();
        let width = map[0].len();
        let height = map.len();
        Self {
            ground: map,
            width,
            height,
            start_position,
        }
    }
}

impl Index<Position> for Map {
    type Output = Ground;

    fn index(&self, index: Position) -> &Self::Output {
        &self.ground[index.y][index.x]
    }
}

fn step(map: &Map, positions: &HashSet<Position>) -> HashSet<Position> {
    let mut new_positions = HashSet::new();
    for position in positions {
        let mut new_position = *position;
        new_position.x += 1;
        if new_position.x < map.width && map[new_position] == Ground::Garden {
            new_positions.insert(new_position);
        }
        new_position.x -= 2;
        if map[new_position] == Ground::Garden {
            new_positions.insert(new_position);
        }
        new_position.x += 1;
        new_position.y += 1;
        if new_position.y < map.height && map[new_position] == Ground::Garden {
            new_positions.insert(new_position);
        }
        new_position.y -= 2;
        if map[new_position] == Ground::Garden {
            new_positions.insert(new_position);
        }
    }
    new_positions
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input, 64);
    println!("part 1: {}", result);
}

fn part1(input: &str, steps: usize) -> usize {
    let map = Map::from(input);
    let mut possible_positions = HashSet::new();
    possible_positions.insert(map.start_position);
    for _ in 0..steps {
        possible_positions = step(&map, &possible_positions);
    }
    map.print(&possible_positions);
    possible_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input, 6), 16);
    }
}
