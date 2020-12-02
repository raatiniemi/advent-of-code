use crate::Mode;

pub fn calculate_answer(input: Vec<String>, _mode: Mode) -> Option<i32> {
    let number_of_valid_passwords = input.iter()
        .filter(|v| { is_password_valid(String::from(*v)) })
        .count() as i32;

    return Some(number_of_valid_passwords);
}

fn is_password_valid(value: String) -> bool {
    let segments: Vec<String> = value.split(" ")
        .map(|v| { v.trim_end_matches(":") })
        .map(|v| { String::from(v) })
        .collect();

    let character = String::from(&segments[1]);
    let password = String::from(&segments[2]);
    let length: Vec<String> = segments[0].split('-')
        .map(|v| { String::from(v) })
        .collect();
    let min_length = length[0].parse::<i32>().unwrap();
    let max_length = length[1].parse::<i32>().unwrap();

    let number_of_characters = password.chars()
        .map(|v| { String::from(v) })
        .filter(|v| { character.eq(v) })
        .count() as i32;

    return min_length <= number_of_characters && number_of_characters <= max_length;
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: [&str; 3] = [
        "1-3 a: abcde",
        "1-3 b: cdefg",
        "2-9 c: ccccccccc"
    ];

    #[test]
    fn test_standard_mode() {
        let expected: Option<i32> = Some(2);

        let actual = calculate_answer(
            TEST_INPUT.iter()
                .map(|value| { String::from(*value) })
                .collect(),
            Mode::Standard,
        );

        assert_eq!(expected, actual);
    }
}
