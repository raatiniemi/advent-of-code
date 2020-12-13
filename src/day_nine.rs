use advent_of_code::map_to_i64;

fn calculate_part_one(input: Vec<String>, preamble_count: usize) -> i64 {
    let values = map_to_i64(input);
    return values.iter()
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
        .to_owned();
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
}
