use crate::Difficulty;

pub fn calculate_answer(input: Vec<i32>, difficulty: Difficulty) -> Option<i32> {
    let expected_sum = 2020;
    if difficulty == Difficulty::Bonus {
        with_three_values(&input, expected_sum)
    } else {
        with_two_values(input, expected_sum)
    }
}

fn with_three_values(input: &Vec<i32>, expected_sum: i32) -> Option<i32> {
    let mut value: Option<i32> = None;

    'main: for first in input {
        for second in input {
            for third in input {
                let sum = first + second + third;
                if sum != expected_sum {
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

fn with_two_values(input: Vec<i32>, expected_sum: i32) -> Option<i32> {
    let mut value: Option<i32> = None;

    'main: for lhs in &input {
        for rhs in &input {
            let sum = lhs + rhs;
            if sum != expected_sum {
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
