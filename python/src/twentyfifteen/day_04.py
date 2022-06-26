import itertools
import hashlib
from python.src import lib


def day_four(part: int):
    content = lib.read_input(year=2015, day=4, part=part)
    if part == 1:
        return day_four_calculate_part_one(content)

    return day_four_calculate_part_two(content)


def day_four_calculate_part_one(content: str) -> int:
    for x in itertools.count():
        value = hashlib.md5(bytes(f"{content}{x}", "utf-8")).hexdigest()
        if value.startswith("00000"):
            return x

    return 0


def day_four_calculate_part_two(content: str) -> int:
    for x in itertools.count():
        value = hashlib.md5(bytes(f"{content}{x}", "utf-8")).hexdigest()
        if value.startswith("000000"):
            return x

    return 0
