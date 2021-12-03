fn calculate_part_one(input: Vec<i32>) -> Option<i32> {
    Some(calculate_increments(input))
}

fn calculate_increments(input: Vec<i32>) -> i32 {
    input.windows(2)
        .map(|v| (v.first(), v.last()))
        .map(|(lhs, rhs)| (lhs.unwrap_or(&0), rhs.unwrap_or(&0)))
        .fold(0, |acc, (lhs, rhs)| {
            if lhs < rhs {
                acc + 1
            } else {
                acc
            }
        })
}

fn calculate_part_two(input: Vec<i32>) -> Option<i32> {
    let values: Vec<i32> = input.windows(3)
        .map(|v| v.iter().sum())
        .collect();

    Some(calculate_increments(values))
}

#[cfg(test)]
mod tests {
    use adventofcode::{map_to_i32, read_contents_of_file};

    use super::*;

    #[test]
    fn day_one_part_one_with_empty_input() {
        let input: Vec<i32> = vec![];
        let expected: Option<i32> = Some(0);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_one_with_example() {
        let input = map_to_i32(read_contents_of_file("input/1-example"));
        let expected: Option<i32> = Some(7);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_one_with_input() {
        let input = map_to_i32(read_contents_of_file("input/1"));
        let expected: Option<i32> = Some(1215);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_empty_input() {
        let input: Vec<i32> = vec![];
        let expected: Option<i32> = Some(0);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_example() {
        let input = map_to_i32(read_contents_of_file("input/1-example"));
        let expected: Option<i32> = Some(5);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_input() {
        let input = map_to_i32(read_contents_of_file("input/1"));
        let expected: Option<i32> = Some(1150);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }
}
