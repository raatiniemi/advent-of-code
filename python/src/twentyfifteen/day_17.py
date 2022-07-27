import itertools
import re

from python.src import lib


def calculate(part: int) -> int:
    content = lib.read_input(year=2015, day=17, part=part)
    if part == 1:
        return _calculate_part_one(content)

    return _calculate_part_two(content)


def _calculate_combinations(litre: int, content: str) -> [[int]]:
    matching_combinations = []

    container_volumes = sorted(map(int, content.splitlines()), reverse=True)
    for i in range(len(container_volumes)):
        for combinations in itertools.combinations(container_volumes, i):
            if sum(combinations) == litre:
                matching_combinations.append(combinations)

    return matching_combinations


def _calculate_part_one(content: str) -> int:
    return len(_calculate_combinations(150, content))


def _calculate_part_two(content: str) -> int:
    combinations = _calculate_combinations(150, content)
    buckets = sorted([(x, list(y)) for x, y in itertools.groupby(combinations, key=len)], key=lambda x: x[0])
    return len(list(buckets[0][1]))
