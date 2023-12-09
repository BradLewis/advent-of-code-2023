use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
}

fn part1(input: &str) -> i64 {
    input.lines().map(|l| parse_line(l, &next_value)).sum()
}

fn part2(input: &str) -> i64 {
    input.lines().map(|l| parse_line(l, &prev_value)).sum()
}

fn parse_line(line: &str, f: &dyn Fn(&[i64]) -> i64) -> i64 {
    let numbers: Vec<_> = line
        .split_whitespace()
        .map(|x| x.parse::<i64>().expect("failed to parse int"))
        .collect();

    f(&numbers)
}

fn next_value(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|&x| x == 0) {
        return 0;
    }
    let next_row = next_row(numbers);
    numbers.last().expect("no last entry in numbers") + next_value(&next_row)
}

fn prev_value(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|&x| x == 0) {
        return 0;
    }
    let next_row = next_row(numbers);
    numbers.first().expect("no last entry in numbers") - prev_value(&next_row)
}

fn next_row(numbers: &[i64]) -> Vec<i64> {
    let mut next_row = vec![0; numbers.len() - 1];
    for i in 0..numbers.len() - 1 {
        next_row[i] = numbers[i + 1] - numbers[i];
    }
    next_row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let result = part1(&input);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_parse_line() {
        let result = parse_line("0 3 6 9 12 15", &next_value);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_next_value() {
        let result = next_value(&[1, 1, 1, 1]);
        assert_eq!(result, 1);

        let result = next_value(&[0, 3, 6, 9, 12, 15]);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_next_row() {
        let result = next_row(&[0, 3, 6, 9, 12, 15]);
        assert_eq!(result, vec![3, 3, 3, 3, 3]);

        let result = next_row(&[1, 1, 1, 1]);
        assert_eq!(result, vec![0, 0, 0]);
    }

    #[test]
    fn test_prev_value() {
        let result = prev_value(&[1, 1, 1, 1]);
        assert_eq!(result, 1);

        let result = prev_value(&[0, 3, 6, 9, 12, 15]);
        assert_eq!(result, -3);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let result = part2(&input);
        assert_eq!(result, 2);
    }
}
