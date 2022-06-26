from collections import namedtuple

Point = namedtuple("Point", "x y")


def read_input(year: int, day: int, part: int = 1) -> str:
    file = open(f"input/{year}/day_{day}_{part}", "r")
    content = file.read()
    file.close()

    return content.strip()
