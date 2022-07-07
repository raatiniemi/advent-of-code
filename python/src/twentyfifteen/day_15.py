import math
import re

from python.src import lib

FINISH = 100


def calculate(part: int) -> int:
    content = lib.read_input(year=2015, day=15, part=part)
    if part == 1:
        return _calculate_part_one(content)

    return _calculate_part_two(content)


def _calculate_by_property(percentage_fractions: [int], values_by_property: [int]) -> int:
    return sum(a * b for a, b in zip(percentage_fractions, values_by_property))


def _calculate_scores(percentage_fractions: [int], values_by_properties: [[int]]) -> int:
    return math.prod(
        max(0, _calculate_by_property(percentage_fractions, values_by_property))
        for values_by_property in values_by_properties[:-1]
    )


def _calculate_score_part_one(values_by_properties: [[int]]) -> int:
    current_max_score = 0
    for a in range(FINISH):
        for b in range(FINISH - a):
            for c in range(FINISH - a - b):
                d = FINISH - a - b - c
                score = _calculate_scores([a, b, c, d], values_by_properties)
                current_max_score = max(current_max_score, score)

    return current_max_score


def _calculate_part_one(content: str) -> int:
    data = [list(map(int, re.findall(r'(-?\d)+', line))) for line in content.splitlines()]
    values_by_properties = list(zip(*data))
    return _calculate_score_part_one(values_by_properties)


def _calculate_score_part_two(values_by_properties: [[int]]) -> int:
    current_max_score = 0
    for a in range(FINISH):
        for b in range(FINISH - a):
            for c in range(FINISH - a - b):
                d = FINISH - a - b - c
                x = _calculate_by_property([a, b, c, d], values_by_properties[-1])
                if x <= 500:
                    score = _calculate_scores([a, b, c, d], values_by_properties)
                    current_max_score = max(current_max_score, score)

    return current_max_score


def _calculate_part_two(content: str) -> int:
    data = [list(map(int, re.findall(r'(-?\d)+', line))) for line in content.splitlines()]
    values_by_properties = list(zip(*data))
    return _calculate_score_part_two(values_by_properties)
