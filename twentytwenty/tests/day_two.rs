extern crate twentytwenty;

use adventofcode::{Part, Source};
use twentytwenty::day_two;

#[test]
fn day_two_with_example_for_part_one() {
    let expected = "Day #2 (part one) with example: 2".to_string();

    let actual = day_two::day_two(&Source::Example, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_two_with_input_for_part_one() {
    let expected = "Day #2 (part one) with input: 418".to_string();

    let actual = day_two::day_two(&Source::Input, &Part::One);

    assert_eq!(expected, actual)
}

#[test]
fn day_two_with_example_for_part_two() {
    let expected = "Day #2 (part two) with example: 1".to_string();

    let actual = day_two::day_two(&Source::Example, &Part::Two);

    assert_eq!(expected, actual)
}

#[test]
fn day_two_with_input_for_part_two() {
    let expected = "Day #2 (part two) with input: 616".to_string();

    let actual = day_two::day_two(&Source::Input, &Part::Two);

    assert_eq!(expected, actual)
}
