use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Rock {
    Round,
    Cube,
    Empty,
}

impl From<char> for Rock {
    fn from(c: char) -> Self {
        match c {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            '.' => Rock::Empty,
            _ => panic!("invalid rock type"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Platform {
    rocks: Vec<Vec<Rock>>,
    width: usize,
    height: usize,
}

impl Platform {
    fn new(input: &str) -> Self {
        let rocks: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(Rock::from).collect())
            .collect();
        let width = rocks[0].len();
        let height = rocks.len();
        Self {
            rocks,
            width,
            height,
        }
    }

    fn tilt_north(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.rocks[y][x] != Rock::Round {
                    continue;
                }
                for i in 1..(y + 1) {
                    if self.rocks[y - i][x] != Rock::Empty {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[y - i + 1][x] = Rock::Round;
                        break;
                    }
                    if i == y {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[0][x] = Rock::Round;
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let mut platform = Platform::new(input);
    platform.tilt_north();
    platform
        .rocks
        .iter()
        .enumerate()
        .map(|(i, l)| l.iter().filter(|&&r| r == Rock::Round).count() * (platform.height - i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 136);
    }

    #[test]
    fn test_tilt_north() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let expected_input = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;
        let mut platform = Platform::new(&input);
        platform.tilt_north();
        let expected = Platform::new(&expected_input);
        assert_eq!(platform, expected);
    }
}
