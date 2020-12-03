use crate::Mode;

#[derive(Eq, PartialEq, Clone)]
struct PolicyWithPassword {
    indexes: (i32, i32),
    character: String,
    password: String,
}

pub fn calculate_answer(input: Vec<String>, _mode: Mode) -> Option<i32> {
    let number_of_valid_passwords = input.iter()
        .map(|v| { parse_raw_to_policy_with_password(String::from(v)) })
        .filter(|v| { is_password_valid(v.clone()) })
        .count() as i32;

    return Some(number_of_valid_passwords);
}

fn is_password_valid(configuration: PolicyWithPassword) -> bool {
    let number_of_characters = count_number_of_characters_in_password(
        configuration.password,
        configuration.character,
    );

    let (min_length, max_length) = configuration.indexes;
    return min_length <= number_of_characters && number_of_characters <= max_length;
}

/// Parse the raw value into the `PolicyWithPassword`.
///
/// ```rust
/// let expected = PolicyWithPassword {
///     indexes: (1, 3),
///     character: String::from("a"),
///     password: String::from("abcde")
/// };
/// let actual = parse_raw_to_policy_with_password(String::from("1-3 a: abcde"));
/// assert_eq!(actual, expected);
/// ```
fn parse_raw_to_policy_with_password(raw: String) -> PolicyWithPassword {
    let segments: Vec<String> = split_raw_into_segments(raw);
    if segments.len() != 3 {
        panic!("Expected raw value in form of three segments, found {}", segments.len())
    }

    return PolicyWithPassword {
        indexes: parse_indexes_from_segment(&segments[0]),
        character: String::from(&segments[1]),
        password: String::from(&segments[2]),
    };
}

fn split_raw_into_segments(raw: String) -> Vec<String> {
    raw.split(" ")
        .map(|v| { v.trim_end_matches(":") })
        .map(|v| { String::from(v) })
        .collect()
}

fn parse_indexes_from_segment(segment: &str) -> (i32, i32) {
    let indexes: Vec<i32> = segment.split('-')
        .map(|v| { String::from(v) })
        .map(|v| { v.parse::<i32>().unwrap() })
        .collect();

    if indexes.len() != 2 {
        panic!("Expected indexes to contain two segments, found {}", indexes.len())
    }
    (indexes[0], indexes[1])
}

fn count_number_of_characters_in_password(password: String, character: String) -> i32 {
    let number_of_characters = password.chars()
        .map(|v| { String::from(v) })
        .filter(|v| { character.eq(v) })
        .count() as i32;
    number_of_characters
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
