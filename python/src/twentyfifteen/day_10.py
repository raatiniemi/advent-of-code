import functools

from python.src import lib


def day_ten(part: int) -> int:
    content = lib.read_input(year=2015, day=10, part=part)
    if part == 1:
        return day_ten_calculate_part_one(content)

    return day_ten_calculate_part_two(content)


def calculate_input(content):
    numbers = []

    current_number = content[0]
    current_number_count = 0
    for number in [x for x in content]:
        if current_number != number:
            numbers.append((current_number, current_number_count))
            current_number_count = 0

        current_number = number
        current_number_count += 1

    numbers.append((current_number, current_number_count))

    return ''.join([f"{y}{x}" for x, y in numbers])


def day_ten_calculate_part_one(content: str) -> int:
    return len(functools.reduce(lambda a, _: calculate_input(a), range(0, 40), content))


def day_ten_calculate_part_two(content: str) -> int:
    return len(functools.reduce(lambda a, _: calculate_input(a), range(0, 50), content))
