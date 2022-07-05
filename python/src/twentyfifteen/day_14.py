from python.src import lib


def calculate(part: int) -> int:
    content = lib.read_input(year=2015, day=14, part=part)
    if part == 1:
        return _calculate_part_one(content)

    return _calculate_part_two(content)


def _calculate_reindeer_distance_after_seconds(seconds: int, data: [str]) -> {str, int}:
    reindeer_distance: {str, int} = {}
    for reindeer in [x.split(' ') for x in data]:
        name = reindeer[0]
        speed = int(reindeer[3])
        duration = int(reindeer[6])
        rest = int(reindeer[13])

        repeats, r = divmod(seconds, duration + rest)
        reindeer_distance[name] = speed * (repeats * duration + min(r, duration))

    return reindeer_distance


def _calculate_part_one(content: str) -> int:
    reindeer_distance = _calculate_reindeer_distance_after_seconds(2503, content.splitlines())
    return max(reindeer_distance.values())


def _calculate_reindeer_points_after_seconds(seconds: int, data: [str]) -> {str, int}:
    scores = dict([(line.split(' ')[0], 0) for line in data])
    for time in range(seconds):
        current_distances = _calculate_reindeer_distance_after_seconds(time + 1, data)
        furthest_distance = max(current_distances.values())
        for reindeer in current_distances:
            current_distance = current_distances[reindeer]
            scores[reindeer] += furthest_distance == current_distance

    return scores


def _calculate_part_two(content: str) -> int:
    reindeer_points = _calculate_reindeer_points_after_seconds(2503, content.splitlines())
    return max(reindeer_points.values())
