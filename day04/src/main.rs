use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("unable to load file");
    let result = part1(&content);
    println!("part1: {}", result);
}

fn extract_numbers(numbers: &str) -> HashSet<usize> {
    numbers
        .split(" ")
        .filter_map(|n| {
            if n.is_empty() {
                return None;
            }
            Some(
                n.parse::<usize>()
                    .expect(&format!("failed to parse int {}", n)),
            )
        })
        .collect()
}

fn part1(content: &str) -> usize {
    content
        .lines()
        .into_iter()
        .map(|l| {
            let (_, card) = l.split_once(": ").expect("unable to split card");
            let (winning_str, numbers_str) = card.split_once(" | ").expect("unable to split cards");
            let winning_numbers = extract_numbers(winning_str);
            let numbers = extract_numbers(numbers_str);
            let intersection = winning_numbers.intersection(&numbers).collect::<Vec<_>>();
            if intersection.len() == 0 {
                0
            } else {
                2usize.pow((intersection.len() - 1) as u32)
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let line = "83 86  6 31 17  9 48 53";
        let numbers = extract_numbers(line);

        assert_eq!(numbers, HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]));
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to load file");
        let result = part1(&input);
        assert_eq!(result, 13);
    }
}
