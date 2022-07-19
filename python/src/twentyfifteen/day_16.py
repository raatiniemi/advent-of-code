import re

from python.src import lib

tickers = {
    "children": 3,
    "cats": 7,
    "samoyeds": 2,
    "pomeranians": 3,
    "akitas": 0,
    "vizslas": 0,
    "goldfish": 5,
    "trees": 3,
    "cars": 2,
    "perfumes": 1
}


def calculate(part: int) -> int:
    content = lib.read_input(year=2015, day=16, part=part)
    if part == 1:
        return _calculate_part_one(content)

    return _calculate_part_two(content)


def _extract_properties(line: str) -> [(str, int)]:
    return [(n, int(v)) for n, v in re.findall(r"(\w+): (\d+)", line)]


def _parse_properties(content) -> [[(str, int)]]:
    return [_extract_properties(line) for line in content.splitlines()]


def _match_tickers_part_one(name: str, value: int) -> bool:
    return value == tickers[name]


def _calculate_part_one(content: str) -> int:
    properties_by_sue = _parse_properties(content)
    for which_sue_number, properties in enumerate(properties_by_sue, 1):
        if all([_match_tickers_part_one(name, value) for name, value in properties]):
            return which_sue_number

    return 0


def _match_tickers_part_two(name: str, value: int) -> bool:
    return value > tickers[name] if name in ('cats', 'trees') else \
        value < tickers[name] if name in ('pomeranians', 'goldfish') else \
        value == tickers[name]


def _calculate_part_two(content: str) -> int:
    properties_by_sue = _parse_properties(content)
    for which_sue_number, properties in enumerate(properties_by_sue, 1):
        if all([_match_tickers_part_two(name, value) for name, value in properties]):
            return which_sue_number

    return 0
