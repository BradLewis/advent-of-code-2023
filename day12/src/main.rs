use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
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
            let (springs, criteria) = parse_line1(l);
            let mut cache = HashMap::new();
            number_arrangements(&springs, &criteria, &mut cache)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (mut springs, criteria) = parse_line2(l);
            let mut cache = HashMap::new();
            number_arrangements(&mut springs, &criteria, &mut cache)
        })
        .sum()
}

fn parse_line1(l: &str) -> (Vec<Spring>, Vec<usize>) {
    let (springs_str, criteria_str) = l.split_once(" ").expect("failed to split line");
    let criteria: Vec<_> = criteria_str
        .split(",")
        .map(|s| s.parse::<usize>().expect("failed to parse criteria"))
        .collect();

    let springs: Vec<_> = springs_str.chars().map(Spring::from).collect();
    (springs, criteria)
}

fn parse_line2(l: &str) -> (Vec<Spring>, Vec<usize>) {
    let (springs_str, criteria_str) = l.split_once(" ").expect("failed to split line");
    let criteria_unfolded: Vec<_> = criteria_str
        .split(",")
        .map(|s| s.parse::<usize>().expect("failed to parse criteria"))
        .collect();
    let criteria_len = criteria_unfolded.len();

    let criteria: Vec<_> = criteria_unfolded
        .into_iter()
        .cycle()
        .take(criteria_len * 5)
        .collect();

    let springs: Vec<_> = springs_str
        .chars()
        .map(Spring::from)
        .chain([Spring::Unknown])
        .cycle()
        .take(springs_str.len() * 5 + 4)
        .collect();
    (springs, criteria)
}

fn number_arrangements(
    springs: &[Spring],
    criteria: &[usize],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if criteria.is_empty() {
        if springs.contains(&Spring::Damaged) {
            return 0;
        }
        return 1;
    }

    if springs.len() < criteria.iter().sum::<usize>() + criteria.len() - 1 {
        return 0;
    }

    if let Some(&c) = cache.get(&(springs.len(), criteria.len())) {
        return c;
    }

    let mut count = 0;
    if springs[0] != Spring::Damaged {
        count += number_arrangements(&springs[1..], criteria, cache);
    }
    let criteria_size = criteria[0];
    if springs.len() == criteria_size && !springs.contains(&Spring::Operational) {
        count += 1;
    } else if !springs[..criteria_size].contains(&Spring::Operational)
        && springs[criteria_size] != Spring::Damaged
    {
        count += number_arrangements(&springs[criteria_size + 1..], &criteria[1..], cache);
    }

    cache.insert((springs.len(), criteria.len()), count);
    count
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
        let mut cache = HashMap::new();
        let result = number_arrangements(&row, &criteria, &mut cache);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_unfolded_valid() {
        let (springs, criteria) = parse_line2("???.### 1,1,3");
        let mut cache = HashMap::new();
        let result = number_arrangements(&springs, &criteria, &mut cache);
        assert_eq!(result, 1);

        let (springs, criteria) = parse_line2(".??..??...?##. 1,1,3");
        let mut cache = HashMap::new();
        let result = number_arrangements(&springs, &criteria, &mut cache);
        assert_eq!(result, 16384);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        assert_eq!(part2(&input), 525152);
    }
}
