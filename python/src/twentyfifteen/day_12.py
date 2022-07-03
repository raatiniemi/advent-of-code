import json
from python.src import lib


def day_twelve(part: int) -> int:
    content = lib.read_input(year=2015, day=12, part=part)
    if part == 1:
        return _day_twelve_calculate_part_one(content)

    return _day_twelve_calculate_part_two(content)


def _collect_numbers_part_one(data) -> [int]:
    if type(data) == dict:
        return [x for key in data for x in _collect_numbers_part_one(data[key])]
    elif type(data) == list:
        return [x for value in data for x in _collect_numbers_part_one(value)]
    elif type(data) == str:
        return []
    elif type(data) == int:
        return [data]
    else:
        return []


def _day_twelve_calculate_part_one(content: str) -> int:
    return sum(_collect_numbers_part_one(json.loads(content)))


def _collect_numbers_part_two(data) -> [int]:
    if type(data) == dict:
        if 'red' not in data.values():
            return [x for key in data for x in _collect_numbers_part_two(data[key])]
        else:
            return []
    elif type(data) == list:
        return [x for value in data for x in _collect_numbers_part_two(value)]
    elif type(data) == str:
        return []
    elif type(data) == int:
        return [data]
    else:
        return []


def _day_twelve_calculate_part_two(content: str) -> int:
    return sum(_collect_numbers_part_two(json.loads(content)))
