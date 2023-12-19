#![allow(dead_code)]

use std::{
    fs,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Filled,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(edges: &[Edge], start_pos: Position, width: usize, height: usize) -> Self {
        let mut map = vec![vec![Tile::Empty; width]; height];
        let mut current_pos = start_pos;
        for edge in edges {
            let mut x = current_pos.x;
            let mut y = current_pos.y;
            for _ in 0..edge.length {
                match edge.direction {
                    Direction::Up => y -= 1,
                    Direction::Down => y += 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }
                map[y][x] = Tile::Filled;
            }
            current_pos = Position { x, y };
        }
        Self { map, width, height }
    }

    fn flood_fill(&mut self, position: Position) {
        self[position] = Tile::Filled;
        if position.x > 0 && self[position.up()] == Tile::Empty {
            self.flood_fill(position.up());
        }
        if position.x < self.width - 1 && self[position.down()] == Tile::Empty {
            self.flood_fill(position.down());
        }
        if position.y > 0 && self[position.left()] == Tile::Empty {
            self.flood_fill(position.left());
        }
        if position.y < self.height - 1 && self[position.right()] == Tile::Empty {
            self.flood_fill(position.right());
        }
    }

    fn count_filled(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|tile| **tile == Tile::Filled).count())
            .sum()
    }

    fn print(&self) {
        for row in &self.map {
            for tile in row {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Filled => print!("#"),
                }
            }
            println!();
        }
    }
}

impl Index<Position> for Map {
    type Output = Tile;

    fn index(&self, position: Position) -> &Self::Output {
        &self.map[position.y][position.x]
    }
}

impl IndexMut<Position> for Map {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        &mut self.map[position.y][position.x]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Edge {
    direction: Direction,
    length: usize,
    colour: u32,
}

impl Edge {
    fn new(direction: Direction, length: usize, colour: u32) -> Self {
        Self {
            direction,
            length,
            colour,
        }
    }
}

impl From<&str> for Edge {
    fn from(s: &str) -> Self {
        let mut s = s.splitn(3, " ");
        let direction = Direction::from(s.next().expect("expected direction"));
        let length = s
            .next()
            .expect("expected length")
            .parse::<usize>()
            .expect("expected length to be a number");
        let colour = &s.next().expect("expected colour")[2..8];
        let colour = u32::from_str_radix(colour, 16).expect("expected colour to be a hex number");
        Self::new(direction, length, colour)
    }
}

fn get_bounds(edges: &[Edge]) -> (i32, i32, i32, i32) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut x = 0;
    let mut y = 0;
    for edge in edges {
        match edge.direction {
            Direction::Up => y -= edge.length as i32,
            Direction::Down => y += edge.length as i32,
            Direction::Left => x -= edge.length as i32,
            Direction::Right => x += edge.length as i32,
        }
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    (min_x, min_y, max_x, max_y)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);
}

fn part1(input: &str) -> usize {
    let edges: Vec<_> = input.lines().map(Edge::from).collect();
    let (min_x, min_y, max_x, max_y) = get_bounds(&edges);
    println!("min_x: {}", min_x);
    println!("min_y: {}", min_y);
    println!("max_x: {}", max_x);
    println!("max_y: {}", max_y);
    let width = (max_x - min_x) as usize + 1;
    let height = (max_y - min_y) as usize + 1;
    let start_pos = Position {
        x: -min_x as usize,
        y: -min_y as usize,
    };

    let mut map = Map::new(&edges, start_pos, width, height);
    map.print();

    let fill_start_pos = Position {
        x: start_pos.x + 1,
        y: start_pos.y + 1,
    };
    map.flood_fill(fill_start_pos);
    map.print();

    map.count_filled()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 62);
    }
}
