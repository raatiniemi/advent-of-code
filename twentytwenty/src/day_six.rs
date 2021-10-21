use std::collections::HashSet;

fn calculate_part_one(input: Vec<String>) -> usize {
    input.join::<&str>("\n")
        .split_terminator("\n\n")
        .map(|v| { v.replace("\n", "") })
        .map(|v| count_unique_characters(v))
        .fold(0, |acc, v| acc + v)
}

fn count_unique_characters(v: String) -> usize {
    v.chars().collect::<HashSet<char>>().len()
}

#[cfg(test)]
mod tests {
    use twentytwenty::read_contents_of_file;

    use super::*;

    #[test]
    fn day_six_part_one_with_example() {
        let input = read_contents_of_file("input/6-example");
        let expected: usize = 11;

        let actual = calculate_part_one(input);

        println!("Day #6 (part one) with example: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_six_part_one_with_input() {
        let input = read_contents_of_file("input/6.txt");
        let expected: usize = 6763;

        let actual = calculate_part_one(input);

        println!("Day #6 (part one) with input: {}", actual);
        assert_eq!(expected, actual);
    }
}
