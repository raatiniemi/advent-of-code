import itertools

from python.src import lib


def day_nine(part: int) -> int:
    content = lib.read_input(year=2015, day=9, part=part)
    if part == 1:
        return day_nine_calculate_part_one(content)

    return day_nine_calculate_part_two(content)


def calculate_city_route_distances(content):
    lines = [line for line in content.splitlines()]
    words = [line.split(' ') for line in lines]
    distances = [[(x[0], x[2], int(x[-1])), (x[2], x[0], int(x[-1]))] for x in words]
    city_distances = {(x[0], x[1]): x[2] for distance in distances for x in distance}
    unique_cities = set([c[0] for c in city_distances])

    route_distances = []
    for city_route in itertools.permutations(unique_cities):
        distance = 0
        for i in range(0, len(city_route) - 1):
            from_city = city_route[i]
            to_city = city_route[i + 1]
            distance += city_distances[(from_city, to_city)]

        route_distances.append(distance)

    return route_distances


def day_nine_calculate_part_one(content: str) -> int:
    return min(calculate_city_route_distances(content))


def day_nine_calculate_part_two(content: str) -> int:
    return max(calculate_city_route_distances(content))
