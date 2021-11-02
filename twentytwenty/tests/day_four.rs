extern crate twentytwenty;

use adventofcode::{Part, Source};
use twentytwenty::day_four;

#[test]
fn day_four_with_example_for_part_one() {
    let expected = "Day #4 (part one) with example: 2".to_string();

    let actual = day_four::day_four(&Source::Example, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_four_with_input_for_part_one() {
    let expected = "Day #4 (part one) with input: 245".to_string();

    let actual = day_four::day_four(&Source::Input, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_four_with_example_for_part_two() {
    let expected = "Day #4 (part two) with example: 4".to_string();

    let actual = day_four::day_four(&Source::Example, &Part::Two);

    assert_eq!(expected, actual)
}

#[test]
fn day_four_with_input_for_part_two() {
    let expected = "Day #4 (part two) with input: 133".to_string();

    let actual = day_four::day_four(&Source::Input, &Part::Two);

    assert_eq!(expected, actual)
}
