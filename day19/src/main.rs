use std::fs;

mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1::part1(&input);
    println!("part 1: {result}");

    let result = part2::part2(&input);
    println!("part 2: {result}");
}
