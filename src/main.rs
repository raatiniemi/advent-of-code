mod day_one;

use std::env;
use std::fs;

#[derive(PartialEq)]
pub enum Difficulty {
    Standard,
    Bonus,
}

fn main() {
    let difficulty = read_difficulty();
    let input: Vec<i32> = fs::read_to_string("input/1")
        .expect("Unable to read input file")
        .split("\n")
        .filter(|value| { !value.is_empty() })
        .map(|value| { value.parse::<i32>().unwrap() })
        .collect();

    let value = day_one::calculate_answer(input, difficulty);
    if let Some(answer) = value {
        println!("The answer is: {}", answer);
    } else {
        println!("Unable to calculate an answer");
    }
}

fn read_difficulty() -> Difficulty {
    let args: Vec<String> = env::args()
        .filter(|arg| { arg.contains("bonus") })
        .collect();

    let difficulty: Difficulty;
    if args.is_empty() {
        difficulty = Difficulty::Standard;
    } else {
        difficulty = Difficulty::Bonus;
    }
    difficulty
}
