use crate::Difficulty;

const EXPECTED_SUM: i32 = 2020;

pub fn calculate_answer(input: Vec<i32>, difficulty: Difficulty) -> Option<i32> {
    if difficulty == Difficulty::Bonus {
        with_three_values(&input)
    } else {
        with_two_values(input)
    }
}

fn with_three_values(input: &Vec<i32>) -> Option<i32> {
    let mut value: Option<i32> = None;

    'main: for first in input {
        for second in input {
            for third in input {
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
