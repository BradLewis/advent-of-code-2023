#![allow(dead_code)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    ops::Index,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {result}");
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

fn part1(input: &str) -> usize {
    let map = Map::new(input);
    let start_position = Position { x: 0, y: 0 };
    let target_position = Position {
        x: map.width - 1,
        y: map.height - 1,
    };
    dijkstra(&map, start_position, target_position)
}

fn dijkstra(map: &Map, start_position: Position, target_position: Position) -> usize {
    let mut distances = vec![vec![usize::MAX; map.width]; map.height];
    distances[start_position.y][start_position.x] = 0;
    let mut queue = BinaryHeap::new();
    let start_state = State::new(start_position.x, start_position.y, Direction::Right, 0);
    let mut best = usize::MAX;
    let mut seen = HashSet::new();
    queue.push(Reverse((0, start_state)));
    while let Some(Reverse((_, state))) = queue.pop() {
        seen.insert(state);
        let current_distance = distances[state.position.y][state.position.x];
        let neighbours = get_neighbours(map, &state, 0, 3);
        for neighbour in neighbours {
            if seen.contains(&neighbour) {
                continue;
            }
            let neighbour_distance = distances[neighbour.position.y][neighbour.position.x];
            let distance = current_distance + map[neighbour.position];
            if distance < neighbour_distance {
                if neighbour.position == target_position {
                    println!("found target, {distance}");
                    best = best.min(distance);
                }
                distances[neighbour.position.y][neighbour.position.x] = distance;
                queue.push(Reverse((distance, neighbour)));
            }
        }
    }
    best
}

fn get_neighbours(
    map: &Map,
    state: &State,
    min_same_direction_count: i32,
    max_same_direction_count: i32,
) -> Vec<State> {
    let mut neighbours = Vec::new();
    match state.direction {
        Direction::Up => {
            if state.same_direction_count < min_same_direction_count {
                neighbours.push(State::new(
                    state.position.x,
                    state.position.y - 1,
                    Direction::Up,
                    state.same_direction_count + 1,
                ));
            } else {
                if state.position.y > 0 && state.same_direction_count < max_same_direction_count {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y - 1,
                        Direction::Up,
                        state.same_direction_count + 1,
                    ));
                }
                if state.position.x > min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x - 1,
                        state.position.y,
                        Direction::Left,
                        0,
                    ));
                }
                if state.position.x < map.width - 1 - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x + 1,
                        state.position.y,
                        Direction::Right,
                        0,
                    ));
                }
            }
        }
        Direction::Down => {
            if state.same_direction_count < min_same_direction_count {
                neighbours.push(State::new(
                    state.position.x,
                    state.position.y + 1,
                    Direction::Down,
                    state.same_direction_count + 1,
                ));
            } else {
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
                if state.position.x > min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x - 1,
                        state.position.y,
                        Direction::Left,
                        0,
                    ));
                }
                if state.position.x < map.width - 1 - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x + 1,
                        state.position.y,
                        Direction::Right,
                        0,
                    ));
                }
            }
        }
        Direction::Left => {
            if state.same_direction_count < min_same_direction_count {
                neighbours.push(State::new(
                    state.position.x - 1,
                    state.position.y,
                    Direction::Left,
                    state.same_direction_count + 1,
                ));
            } else {
                if state.position.x > 0 && state.same_direction_count < max_same_direction_count {
                    neighbours.push(State::new(
                        state.position.x - 1,
                        state.position.y,
                        Direction::Left,
                        state.same_direction_count + 1,
                    ));
                }
                if state.position.y > min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y - 1,
                        Direction::Up,
                        0,
                    ));
                }
                if state.position.y < map.height - 1 - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y + 1,
                        Direction::Down,
                        0,
                    ));
                }
            }
        }
        Direction::Right => {
            if state.same_direction_count < min_same_direction_count {
                neighbours.push(State::new(
                    state.position.x + 1,
                    state.position.y,
                    Direction::Right,
                    state.same_direction_count + 1,
                ));
            } else {
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
                if state.position.y > min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y - 1,
                        Direction::Up,
                        0,
                    ));
                }
                if state.position.y < map.height - 1 - min_same_direction_count as usize {
                    neighbours.push(State::new(
                        state.position.x,
                        state.position.y + 1,
                        Direction::Down,
                        0,
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
        assert_eq!(part1(&input), 102);
    }

    #[test]
    fn test_get_neighbours() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let map = Map::new(&input);
        let state = State::new(0, 0, Direction::Right, 0);
        let neighbours = get_neighbours(&map, &state, 0, 3);
        assert_eq!(neighbours.len(), 2);
        assert_eq!(
            neighbours[0],
            State::new(1, 0, Direction::Right, 1),
            "neighbour 0"
        );
        assert_eq!(
            neighbours[1],
            State::new(0, 1, Direction::Down, 0),
            "neighbour 1"
        );

        let state = State::new(5, 3, Direction::Right, 1);
        let neighbours = get_neighbours(&map, &state, 0, 3);
        assert_eq!(neighbours.len(), 3);

        let state = State::new(5, 3, Direction::Right, 3);
        let neighbours = get_neighbours(&map, &state, 0, 3);
        assert_eq!(neighbours.len(), 2);
    }
}
