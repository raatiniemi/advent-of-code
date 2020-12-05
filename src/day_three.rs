use std::ops::Add;

use crate::lib::character_at_index;

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

fn calculate_part_one(input: Vec<String>) -> Option<i32> {
    let slopes: Vec<Point> = Vec::from([
        Point { x: 3, y: 1 }
    ]);

    return calculate_answer(&input, slopes);
}

fn calculate_part_two(input: Vec<String>) -> Option<i32> {
    let walks: Vec<Point> = Vec::from([
        Point { x: 1, y: 1 },
        Point { x: 3, y: 1 },
        Point { x: 5, y: 1 },
        Point { x: 7, y: 1 },
        Point { x: 1, y: 2 },
    ]);

    return calculate_answer(&input, walks);
}

fn calculate_answer(input: &Vec<String>, walks: Vec<Point>) -> Option<i32> {
    return walks.iter()
        .map(|walk| { calculate_number_of_trees(input, *walk) })
        .fold(Some(1), |product, number_of_trees| {
            return match (product, number_of_trees) {
                (Some(lhs), Some(rhs)) => Some(lhs * rhs),
                _ => product
            };
        });
}

fn calculate_number_of_trees(input: &Vec<String>, walk: Point) -> Option<i32> {
    if input.len() == 0 {
        return None;
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
    Some(trees)
}

#[cfg(test)]
mod test {
    use advent_of_code::read_contents_of_file;

    use super::*;

    #[test]
    fn day_three_part_one_with_example() {
        let input: Vec<&str> = Vec::from([
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ]);
        let expected: Option<i32> = Some(7);

        let actual = calculate_part_one(
            input.iter()
                .map(|v| { v.to_string() })
                .collect()
        );

        println!("Day #3 (part one) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_three_part_one_with_input() {
        let input = read_contents_of_file("input/3");
        let expected: Option<i32> = Some(234);

        let actual = calculate_part_one(
            input.iter()
                .map(|v| { v.to_string() })
                .collect()
        );

        println!("Day #3 (part one) with input: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_three_part_two_with_example() {
        let input: Vec<&str> = Vec::from([
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#"
        ]);
        let expected: Option<i32> = Some(336);

        let actual = calculate_part_two(
            input.iter()
                .map(|v| { v.to_string() })
                .collect()
        );

        println!("Day #3 (part two) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }
}
