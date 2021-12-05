use Command::{Down, Forward, Up};

enum Command {
    Forward { x: i32 },
    Down { x: i32 },
    Up { x: i32 },
}

fn calculate_part_one(input: Vec<String>) -> Option<i32> {
    let calculate_position_adjustment = |current_position: (i32, i32), command: Command| -> (i32, i32) {
        let adjustment = match command {
            Forward { x } => (x, 0),
            Down { x } => (0, x),
            Up { x } => (0, -x),
        };

        (current_position.0 + adjustment.0, current_position.1 + adjustment.1)
    };
    let initial_position = (0, 0);
    let (x, y) = input.iter()
        .map(parse_command)
        .fold(initial_position, |current_position, value| {
            match value {
                Some(command) => calculate_position_adjustment(current_position, command),
                None => current_position,
            }
        });

    Some(x * y)
}

fn parse_command(v: &String) -> Option<Command> {
    let mut x = v.splitn(2, ' ');
    let name = x.next().unwrap_or("");
    let value = x.next().unwrap_or("")
        .parse::<i32>()
        .unwrap_or(0);

    parse_raw_command(name, value)
}

fn parse_raw_command(name: &str, value: i32) -> Option<Command> {
    match name {
        "forward" => Some(Forward { x: value }),
        "down" => Some(Down { x: value }),
        "up" => Some(Up { x: value }),
        _ => None
    }
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
