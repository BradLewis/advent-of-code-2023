use std::{fs, ops::Index};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u8>>,
    starting_position: Position,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut starting_position = Position { x: 0, y: 0 };
        let map: Vec<Vec<_>> = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(j, c)| {
                        if c == b'S' {
                            starting_position = Position { x: j, y: i };
                        }
                        c
                    })
                    .collect()
            })
            .collect();
        let width = map.len();
        let height = map[0].len();
        Self {
            map,
            starting_position,
            width,
            height,
        }
    }

    fn starting_direction(&self) -> Direction {
        // check north
        if self.starting_position.y > 0 {
            match self.map[self.starting_position.y - 1][self.starting_position.x] {
                b'|' | b'7' | b'F' => return Direction::North,
                _ => {}
            }
        }
        // check east
        if self.starting_position.x < self.width - 1 {
            match self.map[self.starting_position.y][self.starting_position.x + 1] {
                b'-' | b'7' | b'J' => return Direction::East,
                _ => {}
            }
        }
        // check south
        if self.starting_position.y < self.height - 1 {
            match self.map[self.starting_position.y + 1][self.starting_position.x] {
                b'|' | b'J' | b'L' => return Direction::South,
                _ => {}
            }
        }
        // check west
        if self.starting_position.x > 0 {
            match self.map[self.starting_position.y][self.starting_position.x - 1] {
                b'-' | b'L' | b'F' => return Direction::West,
                _ => {}
            }
        }
        panic!("could not find starting direction");
    }

    fn step(&self, position: Position, direction: Direction) -> (Position, Direction) {
        let new_position = match direction {
            Direction::North => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::East => Position {
                x: position.x + 1,
                y: position.y,
            },
            Direction::South => Position {
                x: position.x,
                y: position.y + 1,
            },
            Direction::West => Position {
                x: position.x - 1,
                y: position.y,
            },
        };

        let new_direction = match self[new_position] {
            b'|' | b'-' => direction,
            b'L' => match direction {
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                _ => panic!("invalid direction"),
            },
            b'J' => match direction {
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                _ => panic!("invalid direction"),
            },
            b'7' => match direction {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                _ => panic!("invalid direction"),
            },
            b'F' => match direction {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                _ => panic!("invalid direction"),
            },
            b'S' => self.starting_direction(),
            _ => panic!("invalid position"),
        };
        (new_position, new_direction)
    }
}

impl Index<Position> for Map {
    type Output = u8;

    fn index(&self, index: Position) -> &Self::Output {
        &self.map[index.y][index.x]
    }
}

fn part1(input: &str) -> usize {
    let map = Map::new(input);
    let mut position = map.starting_position;
    let mut direction = map.starting_direction();

    let mut steps = 0;

    loop {
        steps += 1;
        let (new_position, new_direction) = map.step(position, direction);
        if map[new_position] == b'S' {
            break;
        }
        position = new_position;
        direction = new_direction;
    }

    steps >> 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 8);
    }
}
