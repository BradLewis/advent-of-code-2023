use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = solve(&input, 2);
    println!("part 1: {}", result);

    let result = solve(&input, 1000000);
    println!("part 2: {}", result);
}

fn solve(input: &str, expand_factor: i64) -> i64 {
    let mut galaxies = map_galaxies(input);
    let (empty_rows, empty_columns) = find_empty_rows_and_columns(&galaxies);
    expand_universe(&mut galaxies, &empty_rows, &empty_columns, expand_factor);

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum += (galaxies[i].0 - galaxies[j].0).abs() + (galaxies[i].1 - galaxies[j].1).abs();
        }
    }
    sum
}

fn map_galaxies(input: &str) -> Vec<(i64, i64)> {
    let mut galaxies = Vec::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }
    galaxies
}

fn find_empty_rows_and_columns(galaxies: &[(i64, i64)]) -> (Vec<i64>, Vec<i64>) {
    let mut empty_rows = Vec::new();
    let mut empty_columns = Vec::new();
    for y in 0..galaxies.len() {
        let mut found = false;
        for x in 0..galaxies.len() {
            if galaxies[x].1 == y as i64 {
                found = true;
                break;
            }
        }
        if !found {
            empty_rows.push(y as i64);
        }
    }
    for x in 0..galaxies.len() {
        let mut found = false;
        for y in 0..galaxies.len() {
            if galaxies[y].0 == x as i64 {
                found = true;
                break;
            }
        }
        if !found {
            empty_columns.push(x as i64);
        }
    }
    empty_rows.reverse();
    empty_columns.reverse();
    (empty_rows, empty_columns)
}

fn expand_universe(
    galaxies: &mut [(i64, i64)],
    empty_rows: &[i64],
    empty_columns: &[i64],
    expand_factor: i64,
) {
    for galaxy in galaxies.iter_mut() {
        for (i, &row) in empty_rows.iter().enumerate() {
            if galaxy.1 > row {
                galaxy.1 += (empty_rows.len() as i64 - i as i64) * (expand_factor - 1);
                break;
            }
        }
        for (i, &column) in empty_columns.iter().enumerate() {
            if galaxy.0 > column {
                galaxy.0 += (empty_columns.len() as i64 - i as i64) * (expand_factor - 1);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let result = solve(&input, 2);
        assert_eq!(result, 374);
    }

    #[test]
    fn test_find_empty_rows_and_columns() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let galaxies = map_galaxies(&input);
        let (empty_rows, empty_columns) = find_empty_rows_and_columns(&galaxies);
        assert_eq!(empty_rows, vec![7, 3]);
        assert_eq!(empty_columns, vec![8, 5, 2]);
    }

    #[test]
    fn test_expand_universe() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let mut galaxies = map_galaxies(&input);
        let (empty_rows, empty_columns) = find_empty_rows_and_columns(&galaxies);
        assert_eq!(galaxies[0], (3, 0));
        expand_universe(&mut galaxies, &empty_rows, &empty_columns, 2);
        assert_eq!(galaxies[0], (4, 0));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read input file");
        let result = solve(&input, 2);
        assert_eq!(result, 374);
        let result = solve(&input, 10);
        assert_eq!(result, 1030);
        let result = solve(&input, 100);
        assert_eq!(result, 8410);
    }
}
