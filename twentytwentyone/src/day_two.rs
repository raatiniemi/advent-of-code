fn calculate_part_one(input: Vec<String>) -> Option<i32> {
    let (x, y) = input.iter()
        .map(|v| {
            let mut x = v.splitn(2, ' ');
            (x.next().unwrap_or(""), x.next().unwrap_or("").parse::<i32>().unwrap_or(0))
        })
        .map(|(s, v)| {
            match s {
                "forward" => (v, 0),
                "down" => (0, v),
                "up" => (0, -v),
                _ => (0, 0)
            }
        })
        .fold((0, 0), |acc, f| {
            (acc.0 + f.0, acc.1 + f.1)
        });

    Some(x * y)
}

#[cfg(test)]
mod tests {
    use adventofcode::read_contents_of_file;

    use super::*;

    #[test]
    fn day_two_part_one_with_empty_input() {
        let input: Vec<String> = vec![];
        let expected: Option<i32> = Some(0);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_two_part_one_with_example() {
        let input = read_contents_of_file("input/2-example");
        let expected: Option<i32> = Some(150);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_two_part_one_with_input() {
        let input = read_contents_of_file("input/2");
        let expected: Option<i32> = Some(1580000);

        let actual = calculate_part_one(input);

        assert_eq!(expected, actual);
    }
}
