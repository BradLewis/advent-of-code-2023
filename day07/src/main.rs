use std::{collections::HashMap, fs};

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let result = part1(&input);
    println!("part 1: {}", result);
}

struct Hand {
    bid: usize,
    score: usize,
}

impl Hand {
    fn new(cards: &str, bid: usize) -> Self {
        let score = calculate_score(cards);
        Self { bid, score }
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (cards, bid_str) = l.split_once(" ").expect("invalid input");
            let bid = bid_str.parse::<usize>().expect("invalid bid");
            Hand::new(cards, bid)
        })
        .sorted_by(|a, b| a.score.cmp(&b.score))
        .enumerate()
        .map(|(i, a)| a.bid * (i + 1))
        .sum()
}

fn calculate_score(hand: &str) -> usize {
    let counts: HashMap<char, usize> = hand
        .chars()
        .into_grouping_map_by(|&c| c)
        .fold(0, |c, _, _| c + 1);
    let values = counts.values().sorted().collect_vec();
    let hand_type = match values.as_slice() {
        [5] => 0x7,
        [1, 4] => 0x6,
        [2, 3] => 0x5,
        [1, 1, 3] => 0x4,
        [1, 2, 2] => 0x3,
        [1, 1, 1, 2] => 0x2,
        [1, 1, 1, 1, 1] => 0x1,
        _ => 0x0,
    };
    let score: usize = hand
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let value = match c {
                '2' => 0x1,
                '3' => 0x2,
                '4' => 0x3,
                '5' => 0x4,
                '6' => 0x5,
                '7' => 0x6,
                '8' => 0x7,
                '9' => 0x8,
                'T' => 0x9,
                'J' => 0xA,
                'Q' => 0xB,
                'K' => 0xC,
                'A' => 0xD,
                _ => panic!("invalid card"),
            };
            value * 0x10_usize.pow((4 - i) as u32)
        })
        .sum();

    score + 0x10_usize.pow(5) * hand_type
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_score() {
        assert_eq!(calculate_score("AAAAA"), 0x7DDDDD);
        assert_eq!(calculate_score("AKAAA"), 0x6DCDDD);
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input");
        assert_eq!(part1(&input), 6440);
    }
}
