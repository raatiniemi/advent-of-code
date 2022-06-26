import itertools
from python.src import lib


def day_five(part: int):
    content = lib.read_input(year=2015, day=5, part=part)
    if part == 1:
        return len(
            [
                line
                for line in content.splitlines()
                if day_five_is_string_nice_part_one(line)
            ]
        )

    return len(
        [
            line
            for line in content.splitlines()
            if day_five_is_string_nice_part_two(line)
        ]
    )


def day_five_is_string_nice_part_one(line: str) -> bool:
    contains_required_number_of_vowels = (
            len([x for x in line if x in ["a", "e", "i", "o", "u"]]) >= 3
    )
    contains_consecutive_characters = (
            len([y for y in [len(list(x)) for _, x in itertools.groupby(line)] if y >= 2])
            >= 1
    )
    contains_disallowed_substrings = (
            len([x for x in ["ab", "cd", "pq", "xy"] if x in line]) != 0
    )
    return (
            contains_required_number_of_vowels
            and contains_consecutive_characters
            and not contains_disallowed_substrings
    )


def day_five_is_string_nice_part_two(line: str) -> bool:
    contains_duplicate_character_groups_by_two = any(
        [line[i: i + 2] in line[:i] for i in range(len(line) - 1)]
    )
    contains_single_character_separator_pattern = any(
        lhs == rhs for lhs, rhs in zip(line, line[2:])
    )

    return (
            contains_duplicate_character_groups_by_two
            and contains_single_character_separator_pattern
    )
