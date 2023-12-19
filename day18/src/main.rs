#![allow(dead_code)]

use std::fs;

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
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Edge {
    direction: Direction,
    length: usize,
}

impl Edge {
    fn new(direction: Direction, length: usize) -> Self {
        Self { direction, length }
    }
}

impl From<&str> for Edge {
    fn from(s: &str) -> Self {
        let (direction_str, s) = s.split_once(" ").expect("expected direction and length");
        let direction = Direction::from(direction_str);
        let (length_str, _) = s.split_once(" ").expect("expected length");
        let length = length_str
            .parse::<usize>()
            .expect("expected length to be a number");
        Self::new(direction, length)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vertex {
    position: Position,
}

impl Vertex {
    fn new(position: Position) -> Self {
        Self { position }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
}

fn area(vertices: &[Vertex]) -> i64 {
    let mut area = 0;
    for i in 0..(vertices.len() - 1) {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        area += (v1.position.y + v2.position.y) * (v1.position.x - v2.position.x);
    }
    area / 2
}

fn boundary_length(vertices: &[Vertex]) -> i64 {
    let v1 = vertices[0];
    let v2 = vertices[vertices.len() - 1];
    let mut length = (v1.position.x - v2.position.x).abs() + (v1.position.y - v2.position.y).abs();
    for i in 0..(vertices.len() - 1) {
        let v1 = vertices[i];
        let v2 = vertices[i + 1];
        length += (v1.position.x - v2.position.x).abs();
        length += (v1.position.y - v2.position.y).abs();
    }
    length
}

fn solve(edges: &[Edge]) -> i64 {
    let mut vertices = Vec::with_capacity(edges.len() + 1);

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for edge in edges {
        match edge.direction {
            Direction::Up => y -= edge.length as i64,
            Direction::Down => y += edge.length as i64,
            Direction::Left => x -= edge.length as i64,
            Direction::Right => x += edge.length as i64,
        }
        let vertex = Vertex::new(Position { x, y });
        vertices.push(vertex);
    }

    area(&vertices) + boundary_length(&vertices) / 2 + 1
}

fn part1(input: &str) -> i64 {
    let edges: Vec<_> = input.lines().map(Edge::from).collect();
    solve(&edges)
}

fn part2(input: &str) -> i64 {
    let edges: Vec<_> = input
        .lines()
        .map(|l| {
            let s = l.split(" ").last().expect("expected colour");
            let length =
                usize::from_str_radix(&s[2..7], 16).expect("expected colour to be a hex number");
            let last_digit = &s[7..8];
            let direction = Direction::from(last_digit);
            Edge::new(direction, length)
        })
        .collect();

    solve(&edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part2(&input), 952408144115);
    }
}
