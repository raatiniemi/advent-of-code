use Command::{Down, Forward, Up};

enum Command {
    Forward { value: i32 },
    Down { value: i32 },
    Up { value: i32 },
}

struct Position {
    x: i32,
    y: i32,
}

const EMPTY_POSITION: Position = Position {
    x: 0,
    y: 0,
};

fn calculate_part_one(input: Vec<String>) -> Option<i32> {
    let calculate_position_adjustment = |current: Position, command: Command| -> Position {
        let adjustment = match command {
            Forward { value } => Position { x: value, y: 0 },
            Down { value } => Position { x: 0, y: value },
            Up { value } => Position { x: 0, y: -value },
        };

        Position {
            x: current.x + adjustment.x,
            y: current.y + adjustment.y,
        }
    };
    let position = input.iter()
        .map(parse_command)
        .fold(EMPTY_POSITION, |current, value| {
            match value {
                Some(command) => calculate_position_adjustment(current, command),
                None => current,
            }
        });

    Some(position.x * position.y)
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
        "forward" => Some(Forward { value: value }),
        "down" => Some(Down { value: value }),
        "up" => Some(Up { value: value }),
        _ => None
    }
}

fn calculate_part_two(input: Vec<String>) -> Option<i32> {
    let calculate_position_adjustment = |current: (Position, i32), command: Command| -> (Position, i32) {
        let (position, aim) = current;
        match command {
            Forward { value } => {
                let x = position.x + value;
                let y = position.y + aim * value;

                (Position { x, y }, aim)
            }
            Down { value: x } => {
                (position, aim + x)
            }
            Up { value: x } => {
                (position, aim - x)
            }
        }
    };
    let (position, _) = input.iter()
        .map(parse_command)
        .fold((EMPTY_POSITION, 0), |current, value| {
            match value {
                Some(command) => calculate_position_adjustment(current, command),
                None => current,
            }
        });

    Some(position.x * position.y)
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

    #[test]
    fn day_two_part_two_with_empty_input() {
        let input: Vec<String> = vec![];
        let expected: Option<i32> = Some(0);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_two_part_two_with_example() {
        let input = read_contents_of_file("input/2-example");
        let expected: Option<i32> = Some(900);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn day_two_part_two_with_input() {
        let input = read_contents_of_file("input/2");
        let expected: Option<i32> = Some(1251263225);

        let actual = calculate_part_two(input);

        assert_eq!(expected, actual);
    }
}
