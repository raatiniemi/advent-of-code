from python.src import lib


def day_one(part: int):
    content = lib.read_input(year=2015, day=1, part=part)
    if part == 1:
        return day_one_calculate_part_one(content)

    return day_one_calculate_part_two(content)


def day_one_calculate_part_one(content: str) -> int:
    return content.count("(") - content.count(")")


def day_one_calculate_part_two(content: str) -> int:
    value = 0
    for index in range(0, len(content)):
        character = content[index]
        if character == "(":
            value += 1
        else:
            value -= 1

        if value == -1:
            return index + 1

    return value
