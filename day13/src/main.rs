#![allow(dead_code)]

use std::{cmp, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to load input file");
    let result = part1(&input);
    println!("part 1: {}", result);
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|p| Pattern::new(p))
        .map(|p| p.reflection_value())
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Ash,
    Rock,
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            '.' => Element::Ash,
            '#' => Element::Rock,
            _ => panic!("invalid char"),
        }
    }
}

#[derive(Debug)]
struct Pattern {
    map: Vec<Vec<Element>>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn new(pattern: &str) -> Self {
        let elements: Vec<Vec<_>> = pattern
            .lines()
            .map(|l| l.chars().map(Element::from).collect::<Vec<_>>())
            .collect();

        let height = elements.len();
        let width = elements[0].len();
        Self {
            map: elements,
            width,
            height,
        }
    }

    fn reflection_value(&self) -> usize {
        let vertical = self.vertical_reflection();
        let horizontal = self.horizontal_reflection();
        if let Some(v) = vertical {
            return v;
        }
        if let Some(h) = horizontal {
            return 100 * h;
        }
        panic!("no reflection found");
    }

    fn vertical_reflection(&self) -> Option<usize> {
        for x in 0..(self.width - 1) {
            let width = cmp::min(x + 1, self.width - x - 1);
            let result: usize = (0..width)
                .map(|i| {
                    let left = self.map.iter().map(|r| r[x - i]);
                    let right = self.map.iter().map(|r| r[x + i + 1]);
                    Iterator::zip(left, right).filter(|(l, r)| l != r).count()
                })
                .sum();
            if result == 0 {
                return Some(x + 1);
            }
        }
        None
    }

    fn horizontal_reflection(&self) -> Option<usize> {
        for y in 0..(self.height - 1) {
            let height = cmp::min(y + 1, self.height - y - 1);
            let result: usize = (0..height)
                .map(|i| {
                    let up = self.map[y - i].iter();
                    let down = &self.map[y + i + 1];
                    Iterator::zip(up, down).filter(|(u, d)| u != d).count()
                })
                .sum();
            if result == 0 {
                return Some(y + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to load input file");
        let result = part1(&input);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_reflection() {
        let input = fs::read_to_string("test_input.txt").expect("failed to load input file");
        let (v, h) = input.split_once("\n\n").expect("should be 2 patterns");

        let vertical = Pattern::new(&v);
        assert_eq!(vertical.horizontal_reflection(), None);
        assert_eq!(vertical.vertical_reflection(), Some(5));

        let horizontal = Pattern::new(&h);
        assert_eq!(horizontal.horizontal_reflection(), Some(4));
        assert_eq!(horizontal.vertical_reflection(), None);
    }

    //     #[test]
    //     fn test_bad_reflection() {
    //         let input = r#"...##..##....
    // ..##.###...#.
    // ...#.#.#.###.
    // ..#...#.#####
    // ...####...###
    // ##.######.##.
    // .....##.#.##.
    // ...##.#...#.#
    // ####..#.#####
    // ....#####.##.
    // ...####..#..#
    // .....#.###..#
    // ...#..#.#...#
    // ..#.#.#...##.
    // ....#.#...##.
    // "#;
    //         let pattern = Pattern::new(input);
    //
    //         assert_eq!(pattern.horizontal_reflection(), Some(4));
    //         assert_eq!(pattern.vertical_reflection(), None);
    //     }
}
