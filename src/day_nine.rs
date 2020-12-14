use advent_of_code::map_to_i64;

fn calculate_part_one(input: Vec<String>, preamble_count: usize) -> i64 {
    return find_invalid_number(preamble_count, &map_to_i64(input));
}

fn calculate_part_two(input: Vec<String>, preamble_count: usize) -> i64 {
    let values = &map_to_i64(input);
    let invalid_number = find_invalid_number(preamble_count, values);
    let mut contiguous_set = values.iter()
        .enumerate()
        .filter_map({
            |(index, _)| {
                calculate_contiguous_set_to_invalid_number(&invalid_number, index, &values)
            }
        })
        .flat_map(|v| v.to_vec())
        .collect::<Vec<i64>>();

    contiguous_set.sort();

    let min = contiguous_set.first().unwrap_or(&0).to_owned();
    let max = contiguous_set.last().unwrap_or(&0).to_owned();
    return min + max;
}

fn find_invalid_number(preamble_count: usize, values: &Vec<i64>) -> i64 {
    values.iter()
        .skip(preamble_count)
        .enumerate()
        .filter_map({
            |(index, value)| {
                let preamble = extract_preamble(index, preamble_count, &values);
                if is_sum_for_any_value_in_preamble(value, &preamble) {
                    None
                } else {
                    Some(value.to_owned())
                }
            }
        })
        .collect::<Vec<i64>>()
        .first()
        .unwrap_or(&0)
        .to_owned()
}

fn extract_preamble(current_index: usize, preamble_count: usize, values: &Vec<i64>) -> Vec<i64> {
    return values.iter()
        .skip(current_index)
        .take(preamble_count)
        .map(|v| v.to_owned())
        .collect::<Vec<i64>>();
}

fn is_sum_for_any_value_in_preamble(value: &i64, preamble: &Vec<i64>) -> bool {
    preamble.iter()
        .map(|v| value - v)
        .any(|v| preamble.contains(&v))
}

fn calculate_contiguous_set_to_invalid_number(
    invalid_number: &i64,
    current_index: usize,
    values: &Vec<i64>,
) -> Option<Vec<i64>> {
    let mut current_sum: i64 = 0;
    let result = values.iter()
        .skip(current_index)
        .filter(|v| *v != invalid_number)
        .take_while({
            |v| {
                let sum_with_next_value = current_sum + **v;
                let should_include = sum_with_next_value <= *invalid_number;
                if should_include {
                    current_sum = sum_with_next_value;
                }
                should_include
            }
        })
        .map(|v| v.to_owned())
        .collect();

    if current_sum == *invalid_number {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::read_contents_of_file;

    use super::*;

    #[test]
    fn day_nine_part_one_with_example() {
        let input = read_contents_of_file("input/9-example.txt");
        let expected: i64 = 127;

        let actual = calculate_part_one(input, 5);

        println!("Day #9 (part one) with example: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_nine_part_one_with_input() {
        let input = read_contents_of_file("input/9.txt");
        let expected: i64 = 393911906;

        let actual = calculate_part_one(input, 25);

        println!("Day #9 (part one) with example: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_nine_part_two_with_example() {
        let input = read_contents_of_file("input/9-example.txt");
        let expected: i64 = 62;

        let actual = calculate_part_two(input, 5);

        println!("Day #9 (part two) with example: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_nine_part_two_with_input() {
        let input = read_contents_of_file("input/9.txt");
        let expected: i64 = 59341885;

        let actual = calculate_part_two(input, 25);

        println!("Day #9 (part two) with example: {}", actual);
        assert_eq!(expected, actual);
    }
}
