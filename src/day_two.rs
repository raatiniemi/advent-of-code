use crate::lib::character_at_index;

#[derive(Eq, PartialEq, Clone)]
struct PolicyWithPassword {
    indexes: (i32, i32),
    character: String,
    password: String,
}

fn calculate_answer(input: Vec<String>, is_password_valid: &dyn Fn(PolicyWithPassword) -> bool) -> Option<i32> {
    let number_of_valid_passwords = input.iter()
        .map(|v| { parse_raw_to_policy_with_password(v.to_string()) })
        .filter(|v| { is_password_valid(v.clone()) })
        .count() as i32;

    return Some(number_of_valid_passwords);
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
        .map(|v| { v.to_string() })
        .collect()
}

fn parse_indexes_from_segment(segment: &str) -> (i32, i32) {
    let indexes: Vec<i32> = segment.split('-')
        .map(|v| { v.to_string() })
        .map(|v| {
            v.parse::<i32>()
                .expect("Expect valid i32 value when parsing indexes from segment")
        })
        .collect();

    if indexes.len() != 2 {
        panic!("Expected indexes to contain two segments, found {}", indexes.len())
    }
    (indexes[0], indexes[1])
}

fn is_password_valid_for_part_one(configuration: PolicyWithPassword) -> bool {
    let number_of_characters = count_number_of_characters_in_password(
        configuration.character,
        configuration.password,
    );

    let (min_length, max_length) = configuration.indexes;
    min_length <= number_of_characters && number_of_characters <= max_length
}

fn count_number_of_characters_in_password(character: String, password: String) -> i32 {
    password.chars()
        .map(|v| { v.to_string() })
        .filter(|v| { character.eq(v) })
        .count() as i32
}

fn is_password_valid_for_part_two(configuration: PolicyWithPassword) -> bool {
    let (first_index, second_index) = configuration.indexes;

    let first_character_value = character_at_index(first_index - 1, &configuration.password);
    let second_character_value = character_at_index(second_index - 1, &configuration.password);

    match (first_character_value, second_character_value) {
        (Some(character_at_first_index), Some(character_at_second_index)) => {
            match_exclusive(&character_at_first_index, &character_at_second_index, &configuration.character)
        }
        _ => false
    }
}

/// Check whether the either first_character or second_character matches the character.
fn match_exclusive(character_at_first_index: &String, character_at_second_index: &String, character_to_match: &String) -> bool {
    (character_at_first_index.eq(character_to_match) || character_at_second_index.eq(character_to_match))
        && character_at_first_index.ne(character_at_second_index)
}

#[cfg(test)]
mod test {
    use advent_of_code::read_contents_of_file;

    use super::*;

    #[test]
    fn day_two_part_one_with_example() {
        let input: Vec<String> = Vec::from([
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string()
        ]);
        let expected: Option<i32> = Some(2);

        let actual = calculate_answer(input, &is_password_valid_for_part_one);

        println!("Day #2 (part one) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_two_part_one_with_input() {
        let input = read_contents_of_file("input/2");
        let expected: Option<i32> = Some(418);

        let actual = calculate_answer(input, &is_password_valid_for_part_one);

        println!("Day #2 (part one) with input: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_two_part_two_with_example() {
        let input: Vec<String> = Vec::from([
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string()
        ]);
        let expected: Option<i32> = Some(1);

        let actual = calculate_answer(input, &is_password_valid_for_part_two);

        println!("Day #2 (part two) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_two_part_two_with_input() {
        let input = read_contents_of_file("input/2");
        let expected: Option<i32> = Some(616);

        let actual = calculate_answer(input, &is_password_valid_for_part_two);

        println!("Day #2 (part two) with input: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }
}
