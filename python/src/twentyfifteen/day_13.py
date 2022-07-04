from itertools import permutations
from python.src import lib


def day_thirteen(part: int) -> int:
    content = lib.read_input(year=2015, day=13, part=part)
    if part == 1:
        return _day_thirteen_calculate_part_one(content)

    return _day_thirteen_calculate_part_two(content)


def _extract_unit_for_neighbors(data: str) -> ((str, str), int):
    words = data.strip('.').split(' ')
    if words[2] == 'gain':
        points = int(f"{words[3]}")
    else:
        points = int(f"-{words[3]}")

    return (words[0], words[-1]), points


def _extract_units_for_neighbors(data: [str]) -> {(str, str), int}:
    return dict([_extract_unit_for_neighbors(x) for x in data])


def _day_thirteen_calculate_part_one(content: str) -> int:
    units_by_neighbors = _extract_units_for_neighbors(content.strip().splitlines())
    participants = set([x for x, _ in units_by_neighbors])

    scores = []
    for permutation in permutations(participants):
        scores_for_permutation = []
        for i in range(0, len(permutation)):
            lhs = permutation[i]
            rhs = permutation[(i + 1) % len(permutation)]
            scores_for_permutation.append(units_by_neighbors.get((lhs, rhs)))
            scores_for_permutation.append(units_by_neighbors.get((rhs, lhs)))

        scores.append(sum(scores_for_permutation))

    return max(scores)


def _day_thirteen_calculate_part_two(content: str) -> int:
    units_by_neighbors = _extract_units_for_neighbors(content.strip().splitlines())
    participants = set([x for x, _ in units_by_neighbors])
    participants.add("me")

    scores = []
    for permutation in permutations(participants):
        scores_for_permutation = []
        for i in range(0, len(permutation)):
            lhs = permutation[i]
            rhs = permutation[(i + 1) % len(permutation)]
            if lhs == "me" or rhs == "me":
                scores_for_permutation.append(0)
                scores_for_permutation.append(0)
            else:
                scores_for_permutation.append(units_by_neighbors.get((lhs, rhs)))
                scores_for_permutation.append(units_by_neighbors.get((rhs, lhs)))

        scores.append(sum(scores_for_permutation))

    return max(scores)
