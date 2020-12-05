use std::env;
use std::fs;

mod lib;
mod day_one;
mod day_two;
mod day_three;

#[derive(PartialEq)]
pub enum Mode {
    Standard,
    Bonus,
}

fn main() {
    let mode = read_mode();
    let input: Vec<String> = fs::read_to_string("input/1")
        .expect("Unable to read input file")
        .split("\n")
        .filter(|value| { !value.is_empty() })
        .map(|value| { String::from(value) })
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
