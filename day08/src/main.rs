use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result2 = part2(&input);
    println!("part 2: {}", result2);
}

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn part1(input: &str) -> usize {
    let (instruction_str, map_str) = input.split_once("\n\n").expect("failed to split input");
    let instructions = parse_instructions(instruction_str);
    let mut current_str = "AAA";

    let map: HashMap<&str, Node> = map_str
        .lines()
        .map(|l| {
            let (name, lr) = l.split_once(" = ").expect("failed to split line");
            let (left_str, right_str) = lr.split_once(", ").expect("failed to split left-right");
            let left = &left_str[1..4];
            let right = &right_str[0..3];
            (name, Node { left, right })
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

fn part2(input: &str) -> usize {
    let (instruction_str, map_str) = input.split_once("\n\n").expect("failed to split input");
    let instructions = parse_instructions(instruction_str);
    let mut current_nodes: Vec<&str> = Vec::new();

    let map: HashMap<&str, Node> = map_str
        .lines()
        .map(|l| {
            let (name, lr) = l.split_once(" = ").expect("failed to split line");
            if name.ends_with("A") {
                current_nodes.push(name);
            }
            let (left_str, right_str) = lr.split_once(", ").expect("failed to split left-right");
            let left = &left_str[1..4];
            let right = &right_str[0..3];
            (name, Node { left, right })
        })
        .collect();

    current_nodes
        .iter()
        .map(|n| find_step_count(&map, &instructions, n))
        .fold(1, |acc, x| lcm(acc, x))
}

fn find_step_count(
    map: &HashMap<&str, Node>,
    instructions: &[Direction],
    start_node: &str,
) -> usize {
    let mut step = 0;
    let mut current_str = start_node;
    loop {
        if current_str.ends_with("Z") {
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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
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

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input3.txt").expect("failed to read input");
        let result = part2(&input);
        assert_eq!(result, 6);
    }
}
