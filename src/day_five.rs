use crate::day_five::Partition::{Higher, Lower};

struct Range {
    min: usize,
    max: usize,
}

enum Partition {
    Lower,
    Higher,
}

fn calculate_part_one(input: Vec<String>) -> Option<usize> {
    return input.iter()
        .filter_map(|v| { calculate_seat_id(v) })
        .max();
}

fn calculate_seat_id(value: &String) -> Option<usize> {
    if value.len() != 10 {
        return None;
    }

    let (row, column) = value.split_at(7);
    let row_index = calculate_row(row);
    let seat_index = calculate_seat(column);

    return Some(row_index.min * 8 + seat_index.min);
}

fn calculate_row(row: &str) -> Range {
    let initial = Range { min: 0, max: 127 };

    row.chars()
        .map(|v| is_lower('F', v))
        .fold(initial, binary_search())
}

fn calculate_seat(column: &str) -> Range {
    let initial = Range { min: 0, max: 7 };

    column.chars()
        .map(|v| is_lower('L', v))
        .fold(initial, binary_search())
}

fn is_lower(low: char, x: char) -> Partition {
    if low.eq(&x) {
        Lower
    } else {
        Higher
    }
}

fn binary_search() -> fn(Range, Partition) -> Range {
    |range, partition| {
        let offset = ((range.max - range.min) + 2) / 2;

        match partition {
            Lower => Range {
                min: range.min,
                max: range.max - offset,
            },
            Higher => Range {
                min: range.min + offset,
                max: range.max,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::read_contents_of_file;

    use super::*;

    #[test]
    fn day_five_part_one_with_example() {
        let input = read_contents_of_file("input/5-example");
        let expected: Option<usize> = Some(820);

        let actual = calculate_part_one(input);

        println!("Day #5 (part one) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_five_part_one_with_input() {
        let input = read_contents_of_file("input/5");
        let expected: Option<usize> = Some(965);

        let actual = calculate_part_one(input);

        println!("Day #5 (part one) with example: {}", actual.unwrap());
        assert_eq!(expected, actual);
    }
}
