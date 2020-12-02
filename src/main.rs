mod day_one;

use std::env;
use std::fs;

#[derive(PartialEq)]
pub enum Mode {
    Standard,
    Bonus,
}

fn main() {
    let mode = read_mode();
    let input: Vec<i32> = fs::read_to_string("input/1")
        .expect("Unable to read input file")
        .split("\n")
        .filter(|value| { !value.is_empty() })
        .map(|value| { value.parse::<i32>().unwrap() })
        .collect();

    let value = day_one::calculate_answer(input, mode);
    if let Some(answer) = value {
        println!("The answer is: {}", answer);
    } else {
        println!("Unable to calculate an answer");
    }
}

fn read_mode() -> Mode {
    let args: Vec<String> = env::args()
        .filter(|arg| { arg.contains("bonus") })
        .collect();

    let difficulty: Mode;
    if args.is_empty() {
        difficulty = Mode::Standard;
    } else {
        difficulty = Mode::Bonus;
    }
    difficulty
}
