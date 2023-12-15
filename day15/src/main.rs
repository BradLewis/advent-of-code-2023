use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {}", result);

    let result = part2(&input);
    println!("part 2: {}", result);
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|s| calculate_hash(s.as_bytes()))
        .sum()
}

fn calculate_hash(input: &[u8]) -> usize {
    let mut current_value: usize = 0;
    for &c in input {
        current_value += c as usize;
        current_value = 17 * current_value;
        current_value = current_value % 256;
    }
    current_value
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    name: String,
    focal_len: usize,
}

impl Lens {
    fn new(name: &str, focal_len: usize) -> Self {
        Self {
            name: name.to_string(),
            focal_len,
        }
    }
}

fn part2(input: &str) -> usize {
    let steps = input.trim().split(",");
    let mut boxes: Vec<Option<Vec<Lens>>> = vec![None; 256];
    for step in steps {
        if step.contains("-") {
            let (name, _) = step.split_once("-").expect("failed to split step");
            for lenses in &mut boxes {
                if let Some(lenses) = lenses {
                    lenses.retain(|l| l.name != name);
                }
            }
        } else {
            let (name, focal_len_str) = step.split_once("=").expect("failed to split step");
            let focal_len = focal_len_str
                .parse::<usize>()
                .expect("failed to parse focal length for =");
            let lens = Lens::new(name, focal_len);
            let hash = calculate_hash(name.as_bytes());
            if let Some(ref mut lenses) = &mut boxes[hash] {
                if let Some(existing_lens_index) = lenses.iter().position(|l| l.name == lens.name) {
                    lenses[existing_lens_index] = lens;
                } else {
                    lenses.push(lens);
                }
            } else {
                boxes[hash] = Some(vec![lens]);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if let Some(lenses) = b {
                Some(
                    lenses
                        .iter()
                        .enumerate()
                        .map(|(j, l)| (i + 1) * (j + 1) * l.focal_len)
                        .sum::<usize>(),
                )
            } else {
                None
            }
        })
        .sum()
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

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let result = part2(&input);
        assert_eq!(result, 145);
    }
}
