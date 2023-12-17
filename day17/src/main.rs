#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
    ops::Index,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = solve(&input, 1, 3);
    println!("part 1: {result}");

    let result = solve(&input, 4, 10);
    println!("part 2: {result}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Index<Position> for Map {
    type Output = usize;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.map[pos.y][pos.x]
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<usize>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("failed to parse char") as usize)
                    .collect()
            })
            .collect();
        let width = map[0].len();
        let height = map.len();
        Self { map, width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    position: Position,
    direction: Direction,
    same_direction_count: i32,
}

impl State {
    fn new(x: usize, y: usize, direction: Direction, same_direction_count: i32) -> Self {
        let pos = Position { x, y };
        Self {
            position: pos,
            direction,
            same_direction_count,
        }
    }
}

fn solve(input: &str, min_same_direction_count: i32, max_same_direction_count: i32) -> usize {
    let map = Map::new(input);
    let start_position = Position { x: 0, y: 0 };
    let target_position = Position {
        x: map.width - 1,
        y: map.height - 1,
    };
    dijkstra(
        &map,
        start_position,
        target_position,
        min_same_direction_count,
        max_same_direction_count,
    )
}

fn dijkstra(
    map: &Map,
    start_position: Position,
    target_position: Position,
    min_same_direction_count: i32,
    max_same_direction_count: i32,
) -> usize {
    let mut distances = HashMap::new();
    let mut queue = BinaryHeap::new();
    let start_state_right = State::new(start_position.x, start_position.y, Direction::Right, 0);
    let start_state_down = State::new(start_position.x, start_position.y, Direction::Down, 0);
    let mut seen = HashSet::new();
    distances.insert(start_state_right, 0);
    distances.insert(start_state_down, 0);
    queue.push(Reverse((0, start_state_right)));
    queue.push(Reverse((0, start_state_down)));
    while let Some(Reverse((current_distance, state))) = queue.pop() {
        seen.insert(state);
        let neighbours = get_neighbours(
            map,
            state,
            min_same_direction_count,
            max_same_direction_count,
        );
        for neighbour in neighbours {
            if seen.contains(&neighbour) {
                continue;
            }
            let distance = current_distance + map[neighbour.position];
            let neighbour_distance = distances.get(&neighbour).unwrap_or(&usize::MAX);
            if distance < *neighbour_distance {
                if neighbour.position == target_position {
                    return distance;
                }
                distances.insert(neighbour, distance);
                queue.push(Reverse((distance, neighbour)));
            }
        }
    }
    usize::MAX
}

fn get_neighbours(
    map: &Map,
    state: State,
    min_same_direction_count: i32,
    max_same_direction_count: i32,
) -> Vec<State> {
    let mut neighbours = Vec::new();
    match state.direction {
        Direction::Up => {
            if state.position.y > 0 && state.same_direction_count < max_same_direction_count {
                neighbours.push(State::new(
                    state.position.x,
                    state.position.y - 1,
                    Direction::Up,
                    state.same_direction_count + 1,
                ));
            }
            if state.same_direction_count >= min_same_direction_count {
                if state.position.x > (min_same_direction_count - 1) as usize {
                    neighbours.push(State::new(
                        state.position.x - 1,
                        state.position.y,
                        Direction::Left,
                        1,
                    ));
                }
                if state.position.x < map.width - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x + 1,
                        state.position.y,
                        Direction::Right,
                        1,
                    ));
                }
            }
        }
        Direction::Down => {
            if state.position.y < map.height - 1
                && state.same_direction_count < max_same_direction_count
            {
                neighbours.push(State::new(
                    state.position.x,
                    state.position.y + 1,
                    Direction::Down,
                    state.same_direction_count + 1,
                ));
            }
            if state.same_direction_count >= min_same_direction_count {
                if state.position.x > (min_same_direction_count - 1) as usize {
                    neighbours.push(State::new(
                        state.position.x - 1,
                        state.position.y,
                        Direction::Left,
                        1,
                    ));
                }
                if state.position.x < map.width - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x + 1,
                        state.position.y,
                        Direction::Right,
                        1,
                    ));
                }
            }
        }
        Direction::Left => {
            if state.position.x > 0 && state.same_direction_count < max_same_direction_count {
                neighbours.push(State::new(
                    state.position.x - 1,
                    state.position.y,
                    Direction::Left,
                    state.same_direction_count + 1,
                ));
            }
            if state.same_direction_count >= min_same_direction_count {
                if state.position.y > (min_same_direction_count - 1) as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y - 1,
                        Direction::Up,
                        1,
                    ));
                }
                if state.position.y < map.height - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y + 1,
                        Direction::Down,
                        1,
                    ));
                }
            }
        }
        Direction::Right => {
            if state.position.x < map.width - 1
                && state.same_direction_count < max_same_direction_count
            {
                neighbours.push(State::new(
                    state.position.x + 1,
                    state.position.y,
                    Direction::Right,
                    state.same_direction_count + 1,
                ));
            }
            if state.same_direction_count >= min_same_direction_count {
                if state.position.y > (min_same_direction_count - 1) as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y - 1,
                        Direction::Up,
                        1,
                    ));
                }
                if state.position.y < map.height - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y + 1,
                        Direction::Down,
                        1,
                    ));
                }
            }
        }
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(solve(&input, 1, 3), 102);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(solve(&input, 4, 10), 94);
    }

    #[test]
    fn test_part2_2() {
        let input = fs::read_to_string("test_input2.txt").expect("failed to read test input file");
        assert_eq!(solve(&input, 4, 10), 71);
    }
}
