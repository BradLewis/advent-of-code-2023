use std::fs;

fn part1(content: &str) {
    let result: u32 = content
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .map(|l| 10 * l.first().expect("number expected") + l.last().expect("number expected"))
        .sum();
    println!("part1: {}", result);
}

fn part2(content: &str) {
    let result: u32 = content
        .lines()
        .map(|l| {
            l.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|l| 10 * l.first().expect("number expected") + l.last().expect("number expected"))
        .sum();

    println!("part2: {}", result);
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("failed to load input file");
    part1(&content);
    part2(&content);
}
