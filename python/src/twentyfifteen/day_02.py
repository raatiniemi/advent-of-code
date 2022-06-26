import operator
import functools
from python.src import lib


def day_two(part: int):
    content = lib.read_input(year=2015, day=2, part=part)
    if part == 1:
        return day_two_calculate_part_one(content)

    return day_two_calculate_part_two(content)


def day_two_calculate_part_one(content: str) -> int:
    result = 0
    for line in content.splitlines():
        (l, h, w) = [int(x) for x in line.split("x")]
        sides = sorted([l * w, w * h, h * l])
        result += sum([2 * x for x in sides]) + min(sides)

    return result


def day_two_calculate_part_two(content: str) -> int:
    result = 0
    for line in content.splitlines():
        values = sorted([int(x) for x in line.split("x")])
        ribbon = sum(values[:2]) * 2
        result += functools.reduce(operator.mul, values) + ribbon

    return result
