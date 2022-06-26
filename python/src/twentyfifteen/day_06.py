from python.src import lib
from python.src.lib import Point


def day_six(part: int):
    content = lib.read_input(year=2015, day=6, part=part)
    if part == 1:
        return day_six_calculate_part_one(content)

    return day_six_calculate_part_two(content)


def day_six_calculate_part_one(content: str) -> int:
    instructions: (str, range, range) = []
    for line in content.splitlines():
        words = line.split()
        x1, y1 = map(int, words[-3].split(","))
        x2, y2 = map(int, words[-1].split(","))
        instructions.append(
            (words[len(words) == 5], range(x1, x2 + 1), range(y1, y2 + 1))
        )

    lights: dict[Point, int] = {}
    for action, xs, ys in instructions:
        for x in xs:
            for y in ys:
                p = Point(x, y)
                if action == "on":
                    lights[p] = 1
                elif action == "off":
                    lights[p] = 0
                elif action == "toggle":
                    if lights.get(p) == 1:
                        lights[p] = 0
                    else:
                        lights[p] = 1

    return sum(lights.values())


def day_six_calculate_part_two(content: str) -> int:
    instructions: (str, range, range) = []
    for line in content.splitlines():
        words = line.split()
        x1, y1 = map(int, words[-3].split(","))
        x2, y2 = map(int, words[-1].split(","))
        instructions.append(
            (words[len(words) == 5], range(x1, x2 + 1), range(y1, y2 + 1))
        )

    lights: dict[Point, int] = {}
    for action, xs, ys in instructions:
        for x in xs:
            for y in ys:
                p = Point(x, y)
                value = lights.get(p)
                if value is None:
                    value = 0

                if action == "on":
                    lights[p] = value + 1
                elif action == "off":
                    lights[p] = max(0, value - 1)
                elif action == "toggle":
                    lights[p] = value + 2

    return sum(lights.values())
