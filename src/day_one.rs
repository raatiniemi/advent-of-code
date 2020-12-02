use crate::Mode;

const EXPECTED_SUM: i32 = 2020;

pub fn calculate_answer(input: Vec<i32>, mode: Mode) -> Option<i32> {
    if mode == Mode::Bonus {
        with_three_values(input)
    } else {
        with_two_values(input)
    }
}

fn with_three_values(input: Vec<i32>) -> Option<i32> {
    let mut value: Option<i32> = None;

    'main: for first in &input {
        for second in &input {
            for third in &input {
                let sum = first + second + third;
                if sum != EXPECTED_SUM {
                    continue;
                }

                let answer = first * second * third;
                println!("{} + {} + {} = 2020", first, second, third);
                println!("{} * {} * {} = {}", first, second, third, answer);
                value = Some(answer);
                break 'main;
            }
        }
    }

    value
}

fn with_two_values(input: Vec<i32>) -> Option<i32> {
    let mut value: Option<i32> = None;

    'main: for lhs in &input {
        for rhs in &input {
            let sum = lhs + rhs;
            if sum != EXPECTED_SUM {
                continue;
            }

            let answer = lhs * rhs;
            println!("{} + {} = 2020", lhs, rhs);
            println!("{} * {} = {}", lhs, rhs, answer);
            value = Some(answer);
            break 'main;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_standard_mode() {
        let expected: Option<i32> = Some(514579);

        let actual = calculate_answer(
            TEST_INPUT.to_vec(),
            Mode::Standard,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bonus_mode() {
        let expected: Option<i32> = Some(241861950);

        let actual = calculate_answer(
            TEST_INPUT.to_vec(),
            Mode::Bonus,
        );

        assert_eq!(expected, actual);
    }
}
