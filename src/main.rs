use std::env;
use std::env::Args;
use std::fs;

use clap::{App, Arg, ArgMatches};

mod lib;
mod day_one;
mod day_two;
mod day_three;

#[derive(PartialEq, Debug)]
pub enum Mode {
    Standard,
    Bonus,
}

fn parse_mode(value: Option<&str>) -> Mode {
    let mode = value.expect("Value for mode")
        .to_lowercase();

    return if mode.eq(&String::from("bonus")) {
        Mode::Bonus
    } else {
        Mode::Standard
    };
}

fn parse_day(value: Option<&str>) -> i32 {
    return value.map(|v| {
        v.parse::<i32>().expect("Expected a valid i32 value for day")
    }).expect("Expected a value for day");
}

fn main() {
    parse_arguments_and_run_solution(env::args());
}

fn parse_arguments_and_run_solution(args: Args) {
    match parse_arguments(args) {
        Some(matches) => run_solution(matches),
        None => panic!("Unable to run solution based on supplied argument(s)")
    };
}

fn parse_arguments(args: Args) -> Option<ArgMatches<'static>> {
    let result = App::new("Advent of code")
        .version("0.0.1")
        .author("Tobias Raatiniemi <raatiniemi@gmail.com>")
        .arg(Arg::with_name("day")
            .help("For which day should we run the solution, e.g. --day 1.")
            .long("day")
            .value_name("DAY")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("mode")
            .help("For a given day, which mode should be used.")
            .short("m")
            .long("mode")
            .value_name("MODE")
            .takes_value(true)
            .required(false)
            .possible_values(&["standard", "bonus"])
            .default_value("standard")
        )
        .get_matches_from_safe(args);

    return match result {
        Ok(matches) => Some(matches),
        Err(error) => panic!("Unable to parse arguments: {}", error.message)
    };
}

fn run_solution(matches: ArgMatches) {
    let day = parse_day(matches.value_of("day"));
    let mode = parse_mode(matches.value_of("mode"));
    let input = parse_input(day);

    let value = calculate_answer(day, mode, input);
    if let Some(answer) = value {
        println!("The answer is: {}", answer);
    } else {
        println!("Unable to calculate an answer");
    }
}

fn parse_input(day: i32) -> Vec<String> {
    return fs::read_to_string(format!("input/{}", day))
        .expect("Unable to read input file")
        .split("\n")
        .filter(|value| { !value.is_empty() })
        .map(|value| { String::from(value) })
        .collect();
}

fn calculate_answer(day: i32, mode: Mode, input: Vec<String>) -> Option<i32> {
    return match day {
        1 => day_one::calculate_answer(input, mode),
        2 => day_two::calculate_answer(input, mode),
        3 => day_three::calculate_answer(input, mode),
        _ => panic!("No solution is available for day {}", day)
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_mode_without_value() {
        parse_mode(None);
    }

    #[test]
    fn parse_mode_without_valid_value() {
        let expected = Mode::Standard;

        let actual = parse_mode(Some("unknown"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mode_with_uppercase_standard() {
        let expected = Mode::Standard;

        let actual = parse_mode(Some("STANDARD"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mode_with_lowercase_standard() {
        let expected = Mode::Standard;

        let actual = parse_mode(Some("standard"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mode_with_mixed_case_standard() {
        let expected = Mode::Standard;

        let actual = parse_mode(Some("sTaNDaRd"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mode_with_uppercase_bonus() {
        let expected = Mode::Bonus;

        let actual = parse_mode(Some("BONUS"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mode_with_lowercase_bonus() {
        let expected = Mode::Bonus;

        let actual = parse_mode(Some("bonus"));

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_mode_with_mixed_case_bonus() {
        let expected = Mode::Bonus;

        let actual = parse_mode(Some("bOnUs"));

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn parse_day_without_value() {
        parse_day(None);
    }

    #[test]
    #[should_panic]
    fn parse_day_without_valid_value() {
        parse_day(Some("unknown"));
    }

    #[test]
    fn parse_day_with_value() {
        let expected: i32 = 3;

        let actual = parse_day(Some("3"));

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn calculate_answer_without_solution() {
        calculate_answer(26, Mode::Standard, Vec::new());
    }
}
