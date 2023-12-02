use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("could not read file");
    part1(&content);
    part2(&content);
}

enum MaxMarbles {
    Red = 12,
    Green = 13,
    Blue = 14,
}

fn part1(content: &str) {
    let result: usize = content
        .lines()
        .filter_map(|l| {
            let (id_element, games_str) = l.split_once(": ").expect("could not split line");
            let (_, id_str) = id_element.split_once(" ").expect("could not split id");
            let id = id_str.parse::<usize>().expect("could not parse id");
            let games = games_str.split("; ").collect::<Vec<&str>>();
            for game in games.iter() {
                let marbles_counts = game.split(", ");
                for marbles in marbles_counts.into_iter() {
                    let (count, colour) = marbles.split_once(" ").expect("could not split marble");
                    let count = count.parse::<usize>().expect("could not parse count");
                    let colour = match colour {
                        "red" => MaxMarbles::Red,
                        "green" => MaxMarbles::Green,
                        "blue" => MaxMarbles::Blue,
                        _ => panic!("unknown colour"),
                    };
                    if count > colour as usize {
                        return None;
                    }
                }
            }
            Some(id)
        })
        .sum();

    println!("result: {}", result);
}

fn part2(content: &str) {
    let result: usize = content
        .lines()
        .map(|l| {
            let (_, games_str) = l.split_once(": ").expect("could not split line");
            let games = games_str.split("; ").collect::<Vec<&str>>();
            let mut max_marbles = [0; 3];
            for game in games.iter() {
                let marbles_counts = game.split(", ");
                for marbles in marbles_counts.into_iter() {
                    let (count, colour) = marbles.split_once(" ").expect("could not split marble");
                    let count = count.parse::<usize>().expect("could not parse count");
                    match colour {
                        "red" => {
                            if count > max_marbles[0] {
                                max_marbles[0] = count;
                            }
                        }
                        "green" => {
                            if count > max_marbles[1] {
                                max_marbles[1] = count;
                            }
                        }
                        "blue" => {
                            if count > max_marbles[2] {
                                max_marbles[2] = count;
                            }
                        }
                        _ => panic!("unknown colour"),
                    };
                }
            }
            let mut result = 1;
            for marble in max_marbles.iter() {
                result *= marble;
            }
            result
        })
        .sum();

    println!("result: {}", result);
}
