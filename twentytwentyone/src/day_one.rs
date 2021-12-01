fn calculate_part_one(input: Vec<i32>) -> Option<i32> {
    let mut result = 0;
    for (index, value) in input.iter().enumerate() {
        if index == 0 {
            continue
        }

        match input.get(index - 1) {
            Some(previous_value) => {
                if previous_value < value {
                    result += 1;
                }
            }
            _ => ()
        }
    }
    Some(result)
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
}
