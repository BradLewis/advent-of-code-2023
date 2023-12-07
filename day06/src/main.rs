use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
}

fn part1(input: &str) -> usize {
    let (time_str, records_str) = input.split_once("\n").expect("needs two lines");
    let times = parse_line(time_str);
    let records = parse_line(records_str);

    times
        .iter()
        .zip(records.iter())
        .map(|(t, r)| number_possible_records(*t as f64, *r as f64))
        .product()
}

fn part2(input: &str) -> usize {
    let (time_str, records_str) = input.split_once("\n").expect("needs two lines");
    let time = parse_line2(time_str);
    let record = parse_line2(records_str);

    number_possible_records(time as f64, record as f64)
}

fn parse_line2(line: &str) -> usize {
    let (_, list) = line.split_once(": ").expect("unable to split times");
    list.split_ascii_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .expect("unable to parse item")
}

fn parse_line(line: &str) -> Vec<usize> {
    let (_, list) = line.split_once(": ").expect("unable to split times");
    list.split_ascii_whitespace()
        .map(|s| s.parse::<usize>().expect("unable to parse item"))
        .collect()
}

fn number_possible_records(time: f64, record: f64) -> usize {
    let epsilon = 1e-10;
    let discriminant = (time * time - 4.0 * record).sqrt();
    let root1 = (time + discriminant) / 2.0 - epsilon;
    let root2 = (time - discriminant) / 2.0 + epsilon;
    (root1.floor() - root2.ceil() + 1.0) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let result = part1(&input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_number_possible_records() {
        assert_eq!(number_possible_records(7.0, 9.0), 4);
        assert_eq!(number_possible_records(15.0, 40.0), 8);
        assert_eq!(number_possible_records(30.0, 200.0), 9);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let result = part2(&input);
        assert_eq!(result, 71503);
    }
}
