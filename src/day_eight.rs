use std::collections::HashSet;
use crate::day_eight::Operation::{NOP, ACC, JMP};

enum Operation {
    NOP,
    ACC,
    JMP,
}

fn parse_operation(segments: &Vec<&str>) -> Operation {
    let operation = segments.first()
        .unwrap_or(&"nop")
        .to_owned()
        .to_string();

    if operation.eq("acc") {
        ACC
    } else if operation.eq("jmp") {
        JMP
    } else {
        NOP
    }
}

struct Instruction {
    operation: Operation,
    argument: i32,
}

fn calculate_part_one(input: Vec<String>) -> i32 {
    let mut executed_indexes: HashSet<usize> = HashSet::new();
    let mut current_index: usize = 0;
    let mut accumulator = 0;
    loop {
        if executed_indexes.contains(&current_index) {
            break;
        }
        executed_indexes.insert(current_index);

        let instruction = parse_instruction(input.get(current_index));
        match instruction.operation {
            ACC => {
                accumulator += instruction.argument;
                current_index += 1;
            }
            JMP => {
                current_index = ((current_index as i32) + instruction.argument) as usize
            }
            NOP => {
                current_index += 1;
            }
        }
    }

    return accumulator;
}

fn parse_instruction(value: Option<&String>) -> Instruction {
    let instruction = value.unwrap_or(&String::from("nop +0")).to_owned();
    let segments = instruction.split(' ').collect::<Vec<_>>();

    Instruction {
        operation: parse_operation(&segments),
        argument: segments.last().unwrap_or(&"+0")
            .to_owned()
            .parse::<i32>()
            .unwrap_or(0),
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::read_contents_of_file;

    use super::*;

    #[test]
    fn day_eight_part_one_with_example() {
        let input = read_contents_of_file("input/8-example.txt");
        let expected: i32 = 5;

        let actual = calculate_part_one(input);

        println!("Day #8 (part one) with example: {}", actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn day_eight_part_one_with_input() {
        let input = read_contents_of_file("input/8.txt");
        let expected: i32 = 1684;

        let actual = calculate_part_one(input);

        println!("Day #8 (part one) with input: {}", actual);
        assert_eq!(expected, actual);
    }
}
