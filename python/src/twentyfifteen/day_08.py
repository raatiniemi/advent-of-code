from python.src import lib


def day_eight(part: int) -> int:
    content = lib.read_input(year=2015, day=8, part=part)
    if part == 1:
        return day_eight_calculate_part_one(content)

    return day_eight_calculate_part_two(content)


def day_eight_calculate_part_one(content: str) -> int:
    return sum([a - b for a, b in [_count_characters_part_one(x) for x in content.splitlines()]])


def _count_characters_part_one(x: str) -> (int, int):
    return len(x), len(eval(x))


def day_eight_calculate_part_two(content: str) -> int:
    return sum([a - b for a, b in [_count_characters_part_two(x) for x in content.splitlines()]])


def _count_characters_part_two(x: str) -> (int, int):
    a = len(x.replace("\\", "\\\\").replace(r'"', r"\"")) + 2
    b = len(x)
    return a, b
