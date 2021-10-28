extern crate twentytwenty;

use adventofcode::{Part, Source};
use twentytwenty::day_one;

#[test]
fn day_one_with_example_for_part_one() {
    let expected = "Day #1 (part one) with example: 514579".to_string();

    let actual = day_one::day_one(&Source::Example, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_one_with_input_for_part_one() {
    let expected = "Day #1 (part one) with input: 326211".to_string();

    let actual = day_one::day_one(&Source::Input, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_one_with_example_for_part_two() {
    let expected = "Day #1 (part two) with example: 241861950".to_string();

    let actual = day_one::day_one(&Source::Example, &Part::Two);

    assert_eq!(expected, actual)
}

#[test]
fn day_one_with_input_for_part_two() {
    let expected = "Day #1 (part two) with input: 131347190".to_string();

    let actual = day_one::day_one(&Source::Input, &Part::Two);

    assert_eq!(expected, actual)
}
