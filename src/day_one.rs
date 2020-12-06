const EXPECTED_SUM: i32 = 2020;

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
    use advent_of_code::read_contents_of_file;

    use super::*;

    #[test]
    fn day_one_part_one_with_example() {
        let input = read_contents_of_file("input/1-example")
            .iter()
            .map(|v| { v.parse::<i32>().unwrap() })
            .collect();
        let expected: Option<i32> = Some(514579);

        let actual = calculate_part_one(input);

        println!("Day #1 (part one) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_one_with_input() {
        let input = read_contents_of_file("input/1")
            .iter()
            .map(|v| { v.parse::<i32>().unwrap() })
            .collect();
        let expected: Option<i32> = Some(326211);

        let actual = calculate_part_one(input);

        println!("Day #1 (part one) with input: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_example() {
        let input = read_contents_of_file("input/1-example")
            .iter()
            .map(|v| { v.parse::<i32>().unwrap() })
            .collect();
        let expected: Option<i32> = Some(241861950);

        let actual = calculate_part_two(input);

        println!("Day #1 (part two) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_one_part_two_with_input() {
        let input = read_contents_of_file("input/1")
            .iter()
            .map(|v| { v.parse::<i32>().unwrap() })
            .collect();
        let expected: Option<i32> = Some(131347190);

        let actual = calculate_part_two(input);

        println!("Day #1 (part two) with input: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }
}
