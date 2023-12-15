use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
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

    fn tilt_west(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.rocks[y][x] != Rock::Round {
                    continue;
                }
                for i in 1..(x + 1) {
                    if self.rocks[y][x - i] != Rock::Empty {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[y][x - i + 1] = Rock::Round;
                        break;
                    }
                    if i == x {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[y][0] = Rock::Round;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                if self.rocks[y][x] != Rock::Round {
                    continue;
                }
                for i in 1..(self.height - y) {
                    if self.rocks[y + i][x] != Rock::Empty {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[y + i - 1][x] = Rock::Round;
                        break;
                    }
                    if i == self.height - y - 1 {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[self.height - 1][x] = Rock::Round;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.height {
            for x in (0..self.width).rev() {
                if self.rocks[y][x] != Rock::Round {
                    continue;
                }
                for i in 1..(self.width - x) {
                    if self.rocks[y][x + i] != Rock::Empty {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[y][x + i - 1] = Rock::Round;
                        break;
                    }
                    if i == self.width - x - 1 {
                        self.rocks[y][x] = Rock::Empty;
                        self.rocks[y][self.width - 1] = Rock::Round;
                    }
                }
            }
        }
    }

    fn cycle(&mut self, count: usize) -> usize {
        let mut cache: Vec<Vec<Vec<Rock>>> = Vec::new();

        for i in 0..count {
            if cache.contains(&self.rocks) {
                let index = cache.iter().position(|c| c == &self.rocks).unwrap();
                let cycle_length = i - index;
                let final_index = index + (count - index) % cycle_length;

                let rocks = cache[final_index].clone();
                return score(&rocks);
            }
            cache.push(self.rocks.clone());

            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }
        0
    }
}

fn score(rocks: &Vec<Vec<Rock>>) -> usize {
    rocks
        .iter()
        .enumerate()
        .map(|(i, l)| l.iter().filter(|&&r| r == Rock::Round).count() * (rocks.len() - i))
        .sum()
}

fn part1(input: &str) -> usize {
    let mut platform = Platform::new(input);
    platform.tilt_north();
    score(&platform.rocks)
}

fn part2(input: &str) -> usize {
    let mut platform = Platform::new(input);
    platform.cycle(1_000_000_000)
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

    #[test]
    fn test_cycle() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let expected_input = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#;
        let mut platform = Platform::new(&input);
        platform.cycle(1);
        let expected = Platform::new(&expected_input);
        assert_eq!(platform, expected);
    }

    #[test]
    fn test_cycle_2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let expected_input = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#;
        let mut platform = Platform::new(&input);
        platform.cycle(2);
        let expected = Platform::new(&expected_input);
        assert_eq!(platform, expected);
    }

    #[test]
    fn test_cycle_3() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let expected_input = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#;
        let mut platform = Platform::new(&input);
        platform.cycle(3);
        let expected = Platform::new(&expected_input);
        assert_eq!(platform, expected);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part2(&input), 64);
    }
}
