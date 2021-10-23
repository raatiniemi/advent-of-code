use std::ops::Add;

use adventofcode::character_at_index;

const CHARACTER_FOR_TREE: &str = "#";

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn calculate_part_one(input: Vec<String>) -> u32 {
    let slopes: Vec<Point> = Vec::from([
        Point { x: 3, y: 1 }
    ]);

    return calculate_answer(&input, slopes);
}

fn calculate_part_two(input: Vec<String>) -> u32 {
    let walks: Vec<Point> = Vec::from([
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ]);

    return calculate_answer(&input, walks);
}

fn calculate_answer(input: &Vec<String>, walks: Vec<Point>) -> u32 {
    let mut result: u32 = 1;
    for walk in walks {
        result *= calculate_number_of_trees(input, walk)
    }
    return result;
}

fn calculate_number_of_trees(input: &Vec<String>, walk: Point) -> u32 {
    if input.len() == 0 {
        return 1;
    }

    let first = input.first()
        .expect("Expected first entry from Vec");
    let max_length = first.len() as i32;

    let mut trees = 0;
    let mut position = Point { x: 0, y: 0 };
    for row in input {
        let character = character_at_index(position.x % max_length, &row).unwrap();
        if character.eq(&CHARACTER_FOR_TREE.to_string()) {
            trees += 1;
        }

        position = position + walk;
    }
    trees
}

#[cfg(test)]
mod test {
    use adventofcode::read_contents_of_file;

    use super::*;

    #[test]
    fn day_three_part_one_with_example() {
        let input = read_contents_of_file("input/3-example");
        let expected: u32 = 7;

        let actual = calculate_part_one(
            input.iter()
                .map(|v| { v.to_string() })
                .collect()
        );

        println!("Day #3 (part one) with example: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_three_part_one_with_input() {
        let input = read_contents_of_file("input/3");
        let expected: u32 = 234;

        let actual = calculate_part_one(
            input.iter()
                .map(|v| { v.to_string() })
                .collect()
        );

        println!("Day #3 (part one) with input: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_three_part_two_with_example() {
        let input = read_contents_of_file("input/3-example");
        let expected: u32 = 336;

        let actual = calculate_part_two(
            input.iter()
                .map(|v| { v.to_string() })
                .collect()
        );

        println!("Day #3 (part two) with example: {}", actual);
        assert_eq!(expected, actual);
    }
}
