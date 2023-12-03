use std::{collections::HashMap, fs, ops::Index};

#[derive(Debug)]
struct Number {
    value: usize,
    row: usize,
    column: usize,
    size: usize,
}

impl Number {
    fn new(value: usize, row: usize, column: usize) -> Self {
        Number {
            value,
            row,
            column,
            size: 1,
        }
    }

    fn add(&mut self, value: usize) {
        self.value = self.value * 10 + value;
        self.size += 1;
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Map {
    fn from_file(file_name: &str) -> Self {
        let mut map: Vec<Vec<char>> = Vec::new();
        let content = fs::read_to_string(file_name).expect("File not found");
        let mut width = 0;
        for line in content.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            width = row.len();
            map.push(row);
        }
        let height = map.len();
        Map { map, height, width }
    }

    fn is_adjacent_to_symbol(&self, number: &Number) -> bool {
        for i in 0..3 {
            if (number.row == 0 && i == 0) || (number.row == self.height - 1 && i == 2) {
                continue;
            }
            let row_index = number.row + i - 1;
            for j in 0..number.size + 2 {
                if (number.column == 0 && j == 0) || (number.column + j - 1 == self.width) {
                    continue;
                }
                let column_index = number.column + j - 1;
                // skip if we're just checking the numbers
                if row_index == number.row
                    && (column_index >= number.column
                        && column_index <= number.column + number.size - 1)
                {
                    continue;
                }
                let value = &self[row_index][column_index];
                if !value.is_ascii_digit() && *value != '.' {
                    return true;
                }
            }
        }
        false
    }

    fn check_and_append_to_gear(&self, number: &Number, gears: &mut HashMap<usize, Vec<usize>>) {
        for i in 0..3 {
            if (number.row == 0 && i == 0) || (number.row == self.height - 1 && i == 2) {
                continue;
            }
            let row_index = number.row + i - 1;
            for j in 0..number.size + 2 {
                if (number.column == 0 && j == 0) || (number.column + j - 1 == self.width) {
                    continue;
                }
                let column_index = number.column + j - 1;
                // skip if we're just checking the numbers
                if row_index == number.row
                    && (column_index >= number.column
                        && column_index <= number.column + number.size - 1)
                {
                    continue;
                }
                let value = &self[row_index][column_index];
                if *value == '*' {
                    let index = row_index * self.width + column_index;
                    if gears.contains_key(&index) {
                        gears.get_mut(&index).unwrap().push(number.value);
                    } else {
                        gears.insert(index, vec![number.value]);
                    }
                }
            }
        }
    }
}

impl Index<usize> for Map {
    type Output = Vec<char>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.map[index]
    }
}

fn main() {
    let map = Map::from_file("input.txt");
    let part1_result = part1(&map);
    println!("Part 1: {}", part1_result);
    let part2_result = part2(&map);
    println!("Part 2: {}", part2_result);
}

fn part1(map: &Map) -> usize {
    let mut numbers: Vec<usize> = Vec::new();
    for (row_index, row) in map.map.iter().enumerate() {
        let mut number: Option<Number> = None;
        for (column_index, value) in row.iter().enumerate() {
            if value.is_ascii_digit() {
                let value = value
                    .to_digit(10)
                    .expect("should be valid digit at this point")
                    as usize;
                match &mut number {
                    Some(ref mut n) => n.add(value),
                    None => number = Some(Number::new(value, row_index, column_index)),
                }
            } else {
                match number {
                    Some(n) => {
                        if map.is_adjacent_to_symbol(&n) {
                            numbers.push(n.value);
                        }
                        number = None
                    }
                    None => {}
                }
            }

            // if we're at the end of the row, check if the number is adjacent to a symbol
            if column_index == map.width - 1 {
                match number {
                    Some(ref n) => {
                        if map.is_adjacent_to_symbol(&n) {
                            numbers.push(n.value);
                        }
                    }
                    None => {}
                }
            }
        }
    }
    numbers.iter().sum()
}

fn part2(map: &Map) -> usize {
    let mut gears: HashMap<usize, Vec<usize>> = HashMap::new();
    for (row_index, row) in map.map.iter().enumerate() {
        let mut number: Option<Number> = None;
        for (column_index, value) in row.iter().enumerate() {
            if value.is_ascii_digit() {
                let value = value
                    .to_digit(10)
                    .expect("should be valid digit at this point")
                    as usize;
                match &mut number {
                    Some(ref mut n) => n.add(value),
                    None => number = Some(Number::new(value, row_index, column_index)),
                }
            } else {
                match number {
                    Some(n) => {
                        map.check_and_append_to_gear(&n, &mut gears);
                        number = None
                    }
                    None => {}
                }
            }

            // if we're at the end of the row, check if the number is adjacent to a symbol
            if column_index == map.width - 1 {
                match number {
                    Some(ref n) => {
                        map.check_and_append_to_gear(&n, &mut gears);
                        number = None;
                    }
                    None => {}
                }
            }
        }
    }
    gears
        .values()
        .filter_map(|v| {
            if v.len() != 2 {
                None
            } else {
                Some(v[0] * v[1])
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_map() {
        let map = Map::from_file("test_input.txt");
        assert_eq!(map[0][0], '4');
        assert_eq!(map[0][1], '6');
        assert_eq!(map[0][2], '7');
        assert_eq!(map[9][7], '8');
        assert_eq!(map[9][8], '.');
        assert_eq!(map[9][9], '.');
        assert_eq!(map.height, 10);
        assert_eq!(map.width, 10);
    }

    #[test]
    fn test_is_adjacent() {
        let map = Map::from_file("test_input.txt");
        let mut number1 = Number::new(6, 4, 0);
        number1.add(1);
        number1.add(7);
        assert_eq!(number1.size, 3);
        assert_eq!(number1.value, 617);
        assert!(map.is_adjacent_to_symbol(&number1));

        let mut number2 = Number::new(5, 9, 5);
        number2.add(9);
        number2.add(8);
        assert_eq!(number2.size, 3);
        assert_eq!(number2.value, 598);
        assert!(map.is_adjacent_to_symbol(&number2));
    }

    #[test]
    fn test_adjacent_right_edge() {
        let map = Map::from_file("test_input.txt");
        let mut number = Number::new(7, 7, 6);
        number.add(5);
        number.add(5);
        number.add(1);
        assert_eq!(number.size, 4);
        assert_eq!(number.value, 7551);
        assert!(map.is_adjacent_to_symbol(&number));
    }

    #[test]
    fn test_part1() {
        let map = Map::from_file("test_input.txt");
        assert_eq!(part1(&map), 4361);
    }

    #[test]
    fn test_part2() {
        let map = Map::from_file("test_input.txt");
        assert_eq!(part2(&map), 467835);
    }
}
