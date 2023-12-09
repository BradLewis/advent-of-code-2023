use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let result = part1(&input);
    println!("part 1: {}", result);
}

struct Node {
    left: String,
    right: String,
}

fn part1(input: &str) -> usize {
    let (instruction_str, map_str) = input.split_once("\n\n").expect("failed to split input");
    let instructions = parse_instructions(instruction_str);
    let mut current_str = "AAA";

    let map: HashMap<String, Node> = map_str
        .lines()
        .map(|l| {
            let (name, lr) = l.split_once(" = ").expect("failed to split line");
            let (left_str, right_str) = lr.split_once(", ").expect("failed to split left-right");
            let left = &left_str[1..4];
            let right = &right_str[0..3];
            (
                name.to_string(),
                Node {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            )
        })
        .collect();

    let mut step = 0;
    loop {
        if current_str == "ZZZ" {
            return step;
        }
        let node = map.get(current_str).expect("failed to get node");
        let direction = instructions[step % instructions.len()];
        match direction {
            Direction::Left => current_str = &node.left,
            Direction::Right => current_str = &node.right,
        }
        step += 1;
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn parse_instructions(instructions: &str) -> Vec<Direction> {
    instructions
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input");
        let result = part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_input2() {
        let input = fs::read_to_string("test_input2.txt").expect("failed to read input");
        let result = part1(&input);
        assert_eq!(result, 6);
    }
}
