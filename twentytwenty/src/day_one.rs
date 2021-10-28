use adventofcode::{map_to_i32, Part, read_contents_of_file, Source};

const EXPECTED_SUM: i32 = 2020;

pub fn day_one(source: &Source, part: &Part) -> String {
    let path = match source {
        Source::Example => "input/1-example",
        Source::Input => "input/1",
    };
    let input = map_to_i32(read_contents_of_file(path));
    let value = match part {
        Part::One => calculate_part_one(input),
        Part::Two => calculate_part_two(input),
    };
    return format!("Day #1 (part {}) with {}: {}", part, source, value.unwrap());
}

fn calculate_part_one(input: Vec<i32>) -> Option<i32> {
    for lhs in &input {
        for rhs in &input {
            let sum = lhs + rhs;
            if sum == EXPECTED_SUM {
                return Some(lhs * rhs);
            }
        }
    }
    None
}

fn calculate_part_two(input: Vec<i32>) -> Option<i32> {
    for first in &input {
        for second in &input {
            for third in &input {
                let sum = first + second + third;
                if sum == EXPECTED_SUM {
                    return Some(first * second * third);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use adventofcode::{map_to_i32, read_contents_of_file};

    use super::*;

    #[test]
    fn day_one_part_one_with_empty_input() {
        let input: Vec<i32> = vec![];
        let expected: Option<i32> = None;

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_one_with_example() {
        let input = map_to_i32(read_contents_of_file("input/1-example"));
        let expected: Option<i32> = Some(514579);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_one_with_input() {
        let input = map_to_i32(read_contents_of_file("input/1"));
        let expected: Option<i32> = Some(326211);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_empty_input() {
        let input: Vec<i32> = vec![];
        let expected: Option<i32> = None;

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_example() {
        let input = map_to_i32(read_contents_of_file("input/1-example"));
        let expected: Option<i32> = Some(241861950);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_input() {
        let input = map_to_i32(read_contents_of_file("input/1"));
        let expected: Option<i32> = Some(131347190);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }
}
