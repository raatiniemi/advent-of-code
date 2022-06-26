from functools import lru_cache

from python.src import lib


def day_seven(part: int) -> int:
    content = lib.read_input(year=2015, day=7, part=part)
    if part == 1:
        return day_seven_calculate_part_one(content)['a']

    return day_seven_calculate_part_two(content)['a']


def _parse_instructions(c: str) -> [str, str]:
    return dict([(w[-1], w[:len(w) - 2]) for w in [line.split() for line in c.splitlines()]])


def _calculate_instructions(instructions: [str, str]):
    @lru_cache
    def resolve_value(k: str) -> int:
        try:
            return int(k)
        except ValueError:
            lhs = instructions[k]

        if len(lhs) == 1:
            return resolve_value(lhs[0])
        elif len(lhs) == 2:
            return 65535 - resolve_value(lhs[1])
        elif len(lhs) == 3:
            if lhs[1] == 'AND':
                return resolve_value(lhs[0]) & resolve_value(lhs[2])
            elif lhs[1] == 'OR':
                return resolve_value(lhs[0]) | resolve_value(lhs[2])
            elif lhs[1] == 'LSHIFT':
                return resolve_value(lhs[0]) << resolve_value(lhs[2])
            elif lhs[1] == 'RSHIFT':
                return resolve_value(lhs[0]) >> resolve_value(lhs[2])

        return 0

    return dict([(key, resolve_value(key)) for key in instructions.keys()])


def day_seven_calculate_part_one(content: str) -> dict[str, int]:
    instructions = _parse_instructions(content)
    return _calculate_instructions(instructions)


def day_seven_calculate_part_two(content: str) -> dict[str, int]:
    instructions = _parse_instructions(content)
    calculated_instructions = _calculate_instructions(instructions)
    instructions['b'] = [str(calculated_instructions['a'])]
    return _calculate_instructions(instructions)
