use std::{
    fs,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn move_in_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Beam {
    position: Position,
    direction: Direction,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    BackMirror,
    ForwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '\\' => Tile::BackMirror,
            '/' => Tile::ForwardMirror,
            '-' => Tile::HorizontalSplitter,
            '|' => Tile::VerticalSplitter,
            _ => panic!("invalid tile"),
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: i64,
    height: i64,
}

impl Map {
    fn new(input: &str) -> Self {
        let tiles: Vec<Vec<_>> = input
            .lines()
            .map(|l| l.chars().map(Tile::from).collect())
            .collect();
        let width = tiles[0].len();
        let height = tiles.len();
        Self {
            tiles,
            width: width as i64,
            height: height as i64,
        }
    }

    fn traverse(&self, energized_map: &mut EnergizedMap, beam: Beam) {
        // don't track the initial beam
        if beam.position.x != -1
            && beam.position.x != self.width
            && beam.position.y != -1
            && beam.position.y != self.height
        {
            if energized_map[beam.position].contains(&beam) {
                return;
            }
            energized_map[beam.position].push(beam);
        }
        let next_position = beam.position.move_in_direction(beam.direction);
        if next_position.x >= self.width || next_position.y >= self.height {
            return;
        }
        if next_position.x < 0 || next_position.y < 0 {
            return;
        }
        match self[next_position] {
            Tile::Empty => {
                let next_beam = Beam {
                    position: next_position,
                    direction: beam.direction,
                };
                self.traverse(energized_map, next_beam);
            }
            Tile::BackMirror => {
                let next_direction = match beam.direction {
                    Direction::North => Direction::West,
                    Direction::East => Direction::South,
                    Direction::South => Direction::East,
                    Direction::West => Direction::North,
                };
                let next_beam = Beam {
                    position: next_position,
                    direction: next_direction,
                };
                self.traverse(energized_map, next_beam);
            }
            Tile::ForwardMirror => {
                let next_direction = match beam.direction {
                    Direction::North => Direction::East,
                    Direction::East => Direction::North,
                    Direction::South => Direction::West,
                    Direction::West => Direction::South,
                };
                let next_beam = Beam {
                    position: next_position,
                    direction: next_direction,
                };
                self.traverse(energized_map, next_beam);
            }
            Tile::HorizontalSplitter => {
                let next_directions = match beam.direction {
                    Direction::North | Direction::South => vec![Direction::West, Direction::East],
                    Direction::East => vec![Direction::East],
                    Direction::West => vec![Direction::West],
                };
                for next_direction in next_directions {
                    let next_beam = Beam {
                        position: next_position,
                        direction: next_direction,
                    };
                    self.traverse(energized_map, next_beam);
                }
            }
            Tile::VerticalSplitter => {
                let next_directions = match beam.direction {
                    Direction::East | Direction::West => vec![Direction::North, Direction::South],
                    Direction::North => vec![Direction::North],
                    Direction::South => vec![Direction::South],
                };
                for next_direction in next_directions {
                    let next_beam = Beam {
                        position: next_position,
                        direction: next_direction,
                    };
                    self.traverse(energized_map, next_beam);
                }
            }
        }
    }
}

impl Index<Position> for Map {
    type Output = Tile;

    fn index(&self, position: Position) -> &Self::Output {
        &self.tiles[position.y as usize][position.x as usize]
    }
}

#[derive(Debug)]
struct EnergizedMap {
    beams: Vec<Vec<Vec<Beam>>>,
}

impl EnergizedMap {
    fn new(width: i64, height: i64) -> Self {
        let beams = vec![vec![Vec::new(); width as usize]; height as usize];
        Self { beams }
    }

    fn print(&self) {
        for row in &self.beams {
            for beam in row {
                if beam.len() == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

impl Index<Position> for EnergizedMap {
    type Output = Vec<Beam>;

    fn index(&self, position: Position) -> &Self::Output {
        &self.beams[position.y as usize][position.x as usize]
    }
}

impl IndexMut<Position> for EnergizedMap {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.beams[position.y as usize][position.x as usize]
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
}

fn find_energized_tile_count(map: &Map, start_beam: Beam) -> usize {
    let mut energized_map = EnergizedMap::new(map.width, map.height);
    map.traverse(&mut energized_map, start_beam);
    energized_map
        .beams
        .iter()
        .flatten()
        .filter(|b| b.len() != 0)
        .count()
}

fn part1(input: &str) -> usize {
    let map = Map::new(input);
    find_energized_tile_count(
        &map,
        Beam {
            position: Position { x: -1, y: 0 },
            direction: Direction::East,
        },
    )
}

fn part2(input: &str) -> usize {
    let map = Map::new(input);
    let mut starts = Vec::new();
    for x in 0..map.width {
        starts.push(Beam {
            position: Position { x, y: -1 },
            direction: Direction::South,
        });
        starts.push(Beam {
            position: Position { x, y: map.height },
            direction: Direction::North,
        });
    }
    for y in 0..map.height {
        starts.push(Beam {
            position: Position { x: -1, y },
            direction: Direction::East,
        });
        starts.push(Beam {
            position: Position { x: map.width, y },
            direction: Direction::West,
        });
    }
    starts
        .iter()
        .map(|&b| find_energized_tile_count(&map, b))
        .max()
        .expect("failed to find max")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part2(&input), 51);
    }
}
