use std::collections::HashMap;

use adventofcode::{Part, read_contents_of_file, Source};

#[derive(Debug)]
struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    county_id: Option<i32>,
}

pub fn day_four(source: &Source, part: &Part) -> String {
    let value = match (source, part) {
        (Source::Example, Part::One) => {
            calculate_answer(
                read_contents_of_file("input/4-example"),
                basic_validation,
            )
        }
        (Source::Example, Part::Two) => {
            calculate_answer(
                read_contents_of_file("input/4-example-part-two.txt"),
                strict_validation,
            )
        }
        (Source::Input, Part::One) => {
            calculate_answer(
                read_contents_of_file("input/4"),
                basic_validation,
            )
        }
        (Source::Input, Part::Two) => {
            calculate_answer(
                read_contents_of_file("input/4"),
                strict_validation,
            )
        }
    };
    return format!("Day #4 (part {}) with {}: {}", part, source, value);
}

fn build_passport(passport: &HashMap<&str, &str>) -> Passport {
    Passport {
        birth_year: if passport.contains_key("byr") {
            passport["byr"].parse::<i32>().ok()
        } else {
            None
        },
        issue_year: if passport.contains_key("iyr") {
            passport["iyr"].parse::<i32>().ok()
        } else {
            None
        },
        expiration_year: if passport.contains_key("eyr") {
            passport["eyr"].parse::<i32>().ok()
        } else {
            None
        },
        height: if passport.contains_key("hgt") {
            Some(passport["hgt"].to_string())
        } else {
            None
        },
        hair_color: if passport.contains_key("hcl") {
            Some(passport["hcl"].to_string())
        } else {
            None
        },
        eye_color: if passport.contains_key("ecl") {
            Some(passport["ecl"].to_string())
        } else {
            None
        },
        passport_id: if passport.contains_key("pid") {
            Some(passport["pid"].to_string())
        } else {
            None
        },
        county_id: if passport.contains_key("cid") {
            passport["cid"].parse::<i32>().ok()
        } else {
            None
        },
    }
}

fn calculate_answer(input: Vec<String>, validator: fn(&Passport) -> bool) -> usize {
    read_passports(&input).iter()
        .filter(|v| validator(v))
        .count()
}

fn basic_validation(passport: &Passport) -> bool {
    passport.birth_year.is_some()
        && passport.issue_year.is_some()
        && passport.expiration_year.is_some()
        && passport.height.is_some()
        && passport.hair_color.is_some()
        && passport.eye_color.is_some()
        && passport.passport_id.is_some()
}

fn strict_validation(passport: &Passport) -> bool {
    let hex_chars = ["a", "b", "c", "d", "e", "f"];
    let available_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let birth_year = validate(passport.birth_year, is_valid_birth_year);
    let issue_year = validate(passport.issue_year, is_valid_issue_year);
    let expiration_year = validate(passport.expiration_year, is_valid_expiration_year);
    let height = validate_ref(&passport.height, |v| { is_valid_height(v) });
    let hair_color = validate_ref(&passport.hair_color, |v| v.starts_with("#") && v.chars().skip(1).all(|c| is_valid_hex_value(hex_chars, c)));
    let eye_color = validate_ref(&passport.eye_color, |v| available_eye_colors.contains(&v.as_str()));
    let passport_id = validate_ref(&passport.passport_id, |v| v.len() == 9 && v.chars().all(char::is_numeric));

    birth_year && issue_year && expiration_year && height && hair_color && eye_color && passport_id
}

fn validate<T>(value: Option<T>, predicate: fn(T) -> bool) -> bool {
    match value {
        Some(v) => predicate(v),
        _ => false
    }
}

fn validate_ref<T>(value: &Option<T>, predicate: impl Fn(&T) -> bool) -> bool {
    match value {
        Some(v) => predicate(v),
        _ => false
    }
}

fn is_valid_birth_year(v: i32) -> bool { v >= 1920 && v <= 2002 }

fn is_valid_issue_year(v: i32) -> bool { v >= 2010 && v <= 2020 }

fn is_valid_expiration_year(v: i32) -> bool { v >= 2020 && v <= 2030 }

fn is_valid_height(v: &String) -> bool {
    if v.ends_with("cm") {
        let height = v.replace("cm", "")
            .parse::<i32>()
            .unwrap_or(0);

        height >= 150 && height <= 193
    } else if v.ends_with("in") {
        let height = v.replace("in", "")
            .parse::<i32>()
            .unwrap_or(0);

        height >= 59 && height <= 76
    } else {
        false
    }
}

fn is_valid_hex_value(hex_chars: [&str; 6], c: char) -> bool {
    c.is_numeric() || hex_chars.contains(&c.to_string().as_str())
}

fn read_passports(input: &Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport: HashMap<&str, &str> = HashMap::new();
    for line in input {
        if line.is_empty() {
            passports.push(build_passport(&passport));
            passport = HashMap::new();
        }

        line.split_whitespace()
            .filter_map(|v| {
                let key_value: Vec<&str> = v.split(":").collect();
                return if key_value.len() == 2 {
                    let key = *key_value.first()
                        .expect("Expect a valid key");
                    let value = *key_value.last()
                        .expect("Expect a valid value");

                    Some((key, value))
                } else {
                    None
                };
            })
            .for_each(|(key, value)| {
                passport.insert(key, value);
            });
    }
    passports.push(build_passport(&passport));

    return passports;
}

#[cfg(test)]
mod tests {
    use adventofcode::read_contents_of_file;

    use super::*;

    #[test]
    fn day_four_part_one_with_example() {
        let input = read_contents_of_file("input/4-example");
        let expected: usize = 2;

        let actual = calculate_answer(input, basic_validation);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_four_part_one_with_input() {
        let input = read_contents_of_file("input/4");
        let expected: usize = 245;

        let actual = calculate_answer(input, basic_validation);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_four_part_two_with_example() {
        let input = read_contents_of_file("input/4-example-part-two.txt");
        let expected: usize = 4;

        let actual = calculate_answer(input, strict_validation);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_four_part_two_with_input() {
        let input = read_contents_of_file("input/4");
        let expected: usize = 133;

        let actual = calculate_answer(input, strict_validation);

        assert_eq!(expected, actual);
    }
}
