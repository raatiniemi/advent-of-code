use adventofcode::character_at_index;

fn calculate_part_one(input: Vec<String>) -> Option<i32> {
    let number_of_rows = input.len() as i32;
    let number_of_columns = input.first().unwrap_or(&"".to_string()).len() as i32;

    let values: Vec<i32> = (0..number_of_columns)
        .map(|column_index| {
            input.iter()
                .map(|row| {
                    let character = character_at_index(column_index, row);
                    ("1" == character.unwrap_or("0".to_string())) as i32
                })
                .sum()
        })
        .collect();
    let gamma_rate = calculate_gamma_rate(number_of_rows, &values);
    let epsilon_rate = calculate_epsilon_rate(number_of_rows, &values);

    Some(gamma_rate * epsilon_rate)
}

fn calculate_gamma_rate(number_of_rows: i32, values: &Vec<i32>) -> i32 {
    let binary_gamma_rate = values.iter()
        .map(|v| {
            if is_most_significant_bit(number_of_rows, v) {
                "1"
            } else {
                "0"
            }
        })
        .collect::<Vec::<&str>>()
        .join("");

    binary_to_decimal(binary_gamma_rate)
}

fn calculate_epsilon_rate(number_of_rows: i32, values: &Vec<i32>) -> i32 {
    let binary_epsilon_rate = values.iter()
        .map(|v| {
            if is_most_significant_bit(number_of_rows, v) {
                "0"
            } else {
                "1"
            }
        })
        .collect::<Vec::<&str>>()
        .join("");

    binary_to_decimal(binary_epsilon_rate)
}

fn is_most_significant_bit(number_of_rows: i32, value: &i32) -> bool {
    *value > (number_of_rows / 2)
}

fn binary_to_decimal(value: String) -> i32 {
    i32::from_str_radix(value.as_str(), 2)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use adventofcode::read_contents_of_file;

    use super::*;

    #[test]
    fn day_three_part_one_with_empty_input() {
        let input: Vec<String> = vec![];
        let expected: Option<i32> = Some(0);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_three_part_one_with_example() {
        let input = read_contents_of_file("input/3-example");
        let expected: Option<i32> = Some(198);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_three_part_one_with_input() {
        let input = read_contents_of_file("input/3");
        let expected: Option<i32> = Some(3847100);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }
}
