from python.src import lib
from python.src.lib import Point


def move_using_arrow_direction(point: Point, direction: str) -> Point:
    if direction == "^":
        return Point(point.x, point.y + 1)
    elif direction == ">":
        return Point(point.x + 1, point.y)
    elif direction == "v":
        return Point(point.x, point.y - 1)
    elif direction == "<":
        return Point(point.x - 1, point.y)

    return point


def day_three(part: int):
    content = lib.read_input(year=2015, day=3, part=part)
    if part == 1:
        return day_three_calculate_part_one(content)

    return day_three_calculate_part_two(content)


def day_three_calculate_part_one(content: str) -> int:
    point = Point(x=0, y=0)
    visited = {point}
    for direction in content:
        point = move_using_arrow_direction(point, direction)
        visited.add(point)

    return len(visited)


def day_three_calculate_part_two(content: str) -> int:
    lhs = Point(x=0, y=0)
    rhs = Point(x=0, y=0)
    visited = {lhs, rhs}
    is_lhs = True
    for direction in content:
        if is_lhs:
            lhs = move_using_arrow_direction(lhs, direction)
            visited.add(lhs)
        else:
            rhs = move_using_arrow_direction(rhs, direction)
            visited.add(rhs)

        is_lhs = not is_lhs

    return len(visited)
