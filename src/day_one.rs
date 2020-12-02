use crate::Mode;

const EXPECTED_SUM: i32 = 2020;

pub fn calculate_answer(input: Vec<String>, mode: Mode) -> Option<i32> {
    let values: Vec<i32> = input.iter()
        .map(|value| { value.parse::<i32>().unwrap() })
        .collect();

    if mode == Mode::Bonus {
        calculate_bonus_answer(values)
    } else {
        calculate_standard_answer(values)
    }
}

fn calculate_bonus_answer(input: Vec<i32>) -> Option<i32> {
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

fn calculate_standard_answer(input: Vec<i32>) -> Option<i32> {
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
            TEST_INPUT.iter()
                .map(|value| { value.to_string() })
                .collect(),
            Mode::Standard,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_bonus_mode() {
        let expected: Option<i32> = Some(241861950);

        let actual = calculate_answer(
            TEST_INPUT.iter()
                .map(|value| { value.to_string() })
                .collect(),
            Mode::Bonus,
        );

        assert_eq!(expected, actual);
    }
}
