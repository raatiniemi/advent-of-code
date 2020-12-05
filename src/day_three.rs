use std::ops::Add;

use crate::lib::character_at_index;
use crate::Mode;

const CHARACTER_FOR_TREE: &str = "#";
const WALK: Point = Point { x: 3, y: 1 };

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

pub fn calculate_answer(input: Vec<String>, _mode: Mode) -> Option<i32> {
    let start_position = Point { x: 0, y: 0 };

    return calculate_number_of_trees(start_position, input);
}

fn calculate_number_of_trees(start_position: Point, input: Vec<String>) -> Option<i32> {
    let first = input.first();
    if first.is_none() {
        return None;
    }

    let max_x = first.unwrap().len() as i32;

    let mut trees = 0;
    let mut position = start_position;
    for row in input {
        if position.y == 0 {
            position = position + WALK;
            continue;
        }

        let character = character_at_index(position.x, &row).unwrap();
        if character.eq(&CHARACTER_FOR_TREE.to_string()) {
            trees += 1;
        }

        position = position + WALK;
        if position.x >= max_x {
            position.x -= max_x;
        }
    }
    Some(trees)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: [&str; 11] = [
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
    ];

    #[test]
    fn test_standard_mode() {
        let expected: Option<i32> = Some(7);

        let actual = calculate_answer(
            TEST_INPUT.iter()
                .map(|value| { String::from(*value) })
                .collect(),
            Mode::Standard,
        );

        assert_eq!(expected, actual);
    }
}
