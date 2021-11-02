use adventofcode::{Part, Source};

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_eight;
mod day_nine;

fn main() {
    let days: Vec<fn(&Source, &Part) -> String> = vec![
        day_one::day_one,
        day_two::day_two,
        day_three::day_three,
        day_four::day_four
    ];
    let variations: Vec<(Source, Part)> = vec![
        (Source::Example, Part::One),
        (Source::Input, Part::One),
        (Source::Example, Part::Two),
        (Source::Input, Part::Two),
    ];

    for day in days {
        for (source, part) in &variations {
            println!("{}", day(source, part));
        }
    }
}
