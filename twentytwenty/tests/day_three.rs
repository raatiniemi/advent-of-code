extern crate twentytwenty;

use adventofcode::{Part, Source};
use twentytwenty::day_three;

#[test]
fn day_three_with_example_for_part_one() {
    let expected = "Day #3 (part one) with example: 7".to_string();

    let actual = day_three::day_three(&Source::Example, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_three_with_input_for_part_one() {
    let expected = "Day #3 (part one) with input: 234".to_string();

    let actual = day_three::day_three(&Source::Input, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_three_with_example_for_part_two() {
    let expected = "Day #3 (part two) with example: 336".to_string();

    let actual = day_three::day_three(&Source::Example, &Part::Two);

    assert_eq!(expected, actual)
}

#[test]
fn day_three_with_input_for_part_two() {
    let expected = "Day #3 (part two) with input: Is not working".to_string();

    let actual = day_three::day_three(&Source::Input, &Part::Two);

    assert_eq!(expected, actual)
}
