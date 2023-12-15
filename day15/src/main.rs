use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);
}

fn part1(input: &str) -> u64 {
    input
        .trim()
        .split(",")
        .map(|s| calculate_hash(s.as_bytes()))
        .sum()
}

fn calculate_hash(input: &[u8]) -> u64 {
    let mut current_value: u64 = 0;
    for &c in input {
        current_value += c as u64;
        current_value = 17 * current_value;
        current_value = current_value % 256;
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let result = part1(&input);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_calculate_hash() {
        let input = "rn=1";
        let result = calculate_hash(input.as_bytes());
        assert_eq!(result, 30);

        let input = "cm-";
        let result = calculate_hash(input.as_bytes());
        assert_eq!(result, 253);
    }
}
