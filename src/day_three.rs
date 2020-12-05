use std::ops::Add;

use crate::lib::character_at_index;
use crate::Mode;

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

pub fn calculate_answer(input: Vec<String>, _mode: Mode) -> Option<i32> {
    let first = input.first();
    if first.is_none() {
        return None;
    }

    let max_x = first.unwrap().len() as i32;
    let character_for_tree = String::from(CHARACTER_FOR_TREE);
    let walk = Point { x: 3, y: 1 };

    let mut trees = 0;
    let mut position = Point { x: 0, y: 0 };

    for row in input {
        if position.y == 0 {
            position = position + walk;
            continue;
        }

        let character = character_at_index(position.x, &row).unwrap();
        if character.eq(&character_for_tree) {
            trees += 1;
        }

        position = position + walk;
        if position.x >= max_x {
            position.x -= max_x;
        }
    }
    return Some(trees);
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
