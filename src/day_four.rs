use std::collections::HashMap;

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

fn calculate_part_one(input: Vec<String>) -> usize {
    read_passports(&input).iter()
        .filter(|v| {
            v.birth_year.is_some()
                && v.issue_year.is_some()
                && v.expiration_year.is_some()
                && v.height.is_some()
                && v.hair_color.is_some()
                && v.eye_color.is_some()
                && v.passport_id.is_some()
        })
        .count()
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
    use advent_of_code::read_contents_of_file;

    use super::*;

    #[test]
    fn day_four_part_one_with_example() {
        let input = read_contents_of_file("input/4-example");
        let expected: usize = 2;

        let actual = calculate_part_one(input);

        println!("Day #4 (part one) with example: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_four_part_one_with_input() {
        let input = read_contents_of_file("input/4");
        let expected: usize = 245;

        let actual = calculate_part_one(input);

        println!("Day #4 (part one) with input: {}", actual);
        assert_eq!(expected, actual);
    }
}
