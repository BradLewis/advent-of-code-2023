use std::{
    fs,
    ops::{Index, IndexMut},
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let (up_count, down_count) = part2(&input);
    println!("part 2: up count {}, down count {}", up_count, down_count);
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
        let width = map[0].len();
        let height = map.len();
        Self {
            map,
            starting_position,
            width,
            height,
        }
    }

    fn starting_direction(&self) -> Joint {
        let mut directions = Vec::new();
        // check north
        if self.starting_position.y > 0 {
            match self.map[self.starting_position.y - 1][self.starting_position.x] {
                b'|' | b'7' | b'F' => directions.push(Direction::North),
                _ => {}
            }
        }
        // check east
        if self.starting_position.x < self.width - 1 {
            match self.map[self.starting_position.y][self.starting_position.x + 1] {
                b'-' | b'7' | b'J' => directions.push(Direction::East),
                _ => {}
            }
        }
        // check south
        if self.starting_position.y < self.height - 1 {
            match self.map[self.starting_position.y + 1][self.starting_position.x] {
                b'|' | b'J' | b'L' => directions.push(Direction::South),
                _ => {}
            }
        }
        // check west
        if self.starting_position.x > 0 {
            match self.map[self.starting_position.y][self.starting_position.x - 1] {
                b'-' | b'L' | b'F' => directions.push(Direction::West),
                _ => {}
            }
        }
        match directions.len() {
            2 => Joint::new(directions[0], directions[1]),
            _ => panic!("invalid starting position"),
        }
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
            b'S' => {
                let joint = self.starting_direction();
                joint.outgoing
            }
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
    let joint = map.starting_direction();
    let mut direction = joint.outgoing;

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

fn part2(input: &str) -> (usize, usize) {
    let map = Map::new(input);
    let direction_map = generate_directional_map(&map);
    count_spins(&direction_map)
}

fn generate_directional_map(map: &Map) -> DirectionMap {
    let mut direction_map = DirectionMap::new(map.height, map.width);
    let mut position = map.starting_position;
    let joint = map.starting_direction();
    let mut direction = joint.outgoing;

    direction_map[position] = Some(joint);

    loop {
        let (new_position, new_direction) = map.step(position, direction);
        if map[new_position] == b'S' {
            break;
        }
        direction_map[new_position] = Some(Joint::new(new_direction, direction));
        position = new_position;
        direction = new_direction;
    }
    direction_map
}

#[derive(Debug)]
struct DirectionMap {
    map: Vec<Vec<Option<Joint>>>,
    height: usize,
    width: usize,
}

impl DirectionMap {
    fn new(height: usize, width: usize) -> Self {
        Self {
            map: vec![vec![None; width]; height],
            height,
            width,
        }
    }
}

impl Index<Position> for DirectionMap {
    type Output = Option<Joint>;

    fn index(&self, index: Position) -> &Self::Output {
        &self.map[index.y][index.x]
    }
}

impl IndexMut<Position> for DirectionMap {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.map[index.y][index.x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spin {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Joint {
    incoming: Direction,
    outgoing: Direction,
}

impl Joint {
    fn new(outgoing: Direction, incoming: Direction) -> Self {
        Self { incoming, outgoing }
    }
}

fn count_spins(map: &DirectionMap) -> (usize, usize) {
    let mut up_count = 0;
    let mut down_count = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let position = Position { x, y };
            match map[position] {
                None => {
                    let spin = check_spin(map, position);
                    if spin == Some(Spin::Up) {
                        up_count += 1;
                    } else if spin == Some(Spin::Down) {
                        down_count += 1;
                    }
                }
                _ => {}
            };
        }
    }
    (up_count, down_count)
}

fn check_spin(map: &DirectionMap, position: Position) -> Option<Spin> {
    let north = check_north(map, position)?;
    let _ = check_south(map, position)?;
    let _ = check_east(map, position)?;
    let _ = check_west(map, position)?;

    match north {
        Joint {
            outgoing: Direction::North,
            incoming: Direction::West,
        } => Some(Spin::Down),
        Joint {
            outgoing: Direction::North,
            incoming: Direction::East,
        } => Some(Spin::Up),
        Joint {
            outgoing: Direction::East,
            incoming: _,
        } => Some(Spin::Up),
        Joint {
            outgoing: Direction::West,
            incoming: _,
        } => Some(Spin::Down),
        _ => panic!("invalid direction"),
    }
}

fn check_north(map: &DirectionMap, position: Position) -> Option<Joint> {
    if position.y == 0 && map[position] == None {
        return None;
    }

    match map[position] {
        None => check_north(
            map,
            Position {
                x: position.x,
                y: position.y - 1,
            },
        ),
        Some(direction) => Some(direction),
    }
}

fn check_south(map: &DirectionMap, position: Position) -> Option<Joint> {
    if position.y == map.height - 1 && map[position] == None {
        return None;
    }

    match map[position] {
        None => check_south(
            map,
            Position {
                x: position.x,
                y: position.y + 1,
            },
        ),
        Some(direction) => Some(direction),
    }
}

fn check_east(map: &DirectionMap, position: Position) -> Option<Joint> {
    if position.x == map.width - 1 && map[position] == None {
        return None;
    }

    match map[position] {
        None => check_east(
            map,
            Position {
                x: position.x + 1,
                y: position.y,
            },
        ),
        Some(direction) => Some(direction),
    }
}

fn check_west(map: &DirectionMap, position: Position) -> Option<Joint> {
    if position.x == 0 && map[position] == None {
        return None;
    }

    match map[position] {
        None => check_west(
            map,
            Position {
                x: position.x - 1,
                y: position.y,
            },
        ),
        Some(direction) => Some(direction),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input2.txt").expect("failed to read test input file");
        assert_eq!(part2(&input), (4, 8));
    }

    #[test]
    fn test_is_inside() {
        let input = fs::read_to_string("test_input2.txt").expect("failed to read test input file");
        let map = Map::new(&input);
        let direction_map = generate_directional_map(&map);
        assert_eq!(
            check_spin(&direction_map, Position { x: 2, y: 6 }),
            Some(Spin::Up)
        );
        assert_eq!(
            check_spin(&direction_map, Position { x: 3, y: 6 }),
            Some(Spin::Up)
        );
        assert_eq!(
            check_spin(&direction_map, Position { x: 3, y: 4 }),
            Some(Spin::Down)
        );
        assert_eq!(check_spin(&direction_map, Position { x: 1, y: 8 }), None);
    }

    #[test]
    fn test_part2_2() {
        let input = fs::read_to_string("test_input3.txt").expect("failed to read test input file");
        assert_eq!(part2(&input), (2, 8));
    }

    #[test]
    fn test_is_inside_2() {
        let input = fs::read_to_string("test_input3.txt").expect("failed to read test input file");
        let map = Map::new(&input);
        let direction_map = generate_directional_map(&map);
        for y in 0..map.height {
            for x in 0..map.width {
                let position = Position { x, y };
                print!("{:?} ", direction_map[position]);
            }
            println!();
        }
        assert_eq!(
            check_spin(&direction_map, Position { x: 3, y: 2 }),
            Some(Spin::Up)
        );
        assert_eq!(
            check_spin(&direction_map, Position { x: 7, y: 4 }),
            Some(Spin::Down)
        );
    }

    #[test]
    fn test_check_north() {
        let input = fs::read_to_string("test_input2.txt").expect("failed to read test input file");
        let map = Map::new(&input);
        let direction_map = generate_directional_map(&map);
        assert_eq!(
            check_north(&direction_map, Position { x: 3, y: 6 }),
            Some(Joint {
                outgoing: Direction::East,
                incoming: Direction::East
            })
        );
    }

    #[test]
    fn test_part2_3() {
        let input = fs::read_to_string("test_input4.txt").expect("failed to read test input file");
        assert_eq!(part2(&input), (10, 0));
    }
}
