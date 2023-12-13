#![allow(dead_code)]

use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("invalid spring char"),
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (springs_str, criteria_str) = l.split_once(" ").expect("failed to split line");
            let criteria: Vec<_> = criteria_str
                .split(",")
                .map(|s| s.parse::<usize>().expect("failed to parse criteria"))
                .collect();

            let springs: Vec<_> = springs_str.chars().map(Spring::from).collect();
            number_arrangements(&springs, &criteria)
        })
        .sum()
}

fn number_arrangements(springs: &Vec<Spring>, criteria: &[usize]) -> usize {
    if let Some(index) = springs.iter().position(|s| *s == Spring::Unknown) {
        let mut springs_with_damaged = springs.clone();
        springs_with_damaged[index] = Spring::Damaged;

        let mut springs_with_operational = springs.clone();
        springs_with_operational[index] = Spring::Operational;

        number_arrangements(&springs_with_damaged, criteria)
            + number_arrangements(&springs_with_operational, criteria)
    } else {
        if valid_arrangement(springs, criteria) {
            1
        } else {
            0
        }
    }
}

fn valid_arrangement(springs: &[Spring], criteria: &[usize]) -> bool {
    springs
        .iter()
        .group_by(|&i| i)
        .into_iter()
        .filter_map(|(&k, g)| match k {
            Spring::Damaged => Some(g.count()),
            _ => None,
        })
        .eq(criteria.iter().copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_number_arrangements() {
        let row = vec![
            Spring::Unknown,
            Spring::Unknown,
            Spring::Unknown,
            Spring::Operational,
            Spring::Damaged,
            Spring::Damaged,
            Spring::Damaged,
        ];
        let criteria = vec![1, 1, 3];
        let result = number_arrangements(&row, &criteria);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_valid_arrangement() {
        let row = vec![
            Spring::Damaged,
            Spring::Operational,
            Spring::Damaged,
            Spring::Operational,
            Spring::Damaged,
            Spring::Damaged,
            Spring::Damaged,
        ];
        let criteria = vec![1, 1, 3];
        let result = valid_arrangement(&row, &criteria);
        assert_eq!(result, true);
    }
}
