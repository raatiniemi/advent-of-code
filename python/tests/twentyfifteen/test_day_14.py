import unittest
from python.src.twentyfifteen import day_14 as day


class TwentyFifteenDayFourteenTestCase(unittest.TestCase):
    def test_part_one_example_one(self):
        expected = {
            "Comet": 1120,
            "Dancer": 1056
        }

        actual = day._calculate_reindeer_distance_after_seconds(
            1_000,
            [
                "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            ]
        )

        self.assertEqual(expected, actual)

    def test_part_one(self):
        expected = 2696

        actual = day.calculate(part=1)

        self.assertEqual(expected, actual)

    def test_part_two_example_one(self):
        expected = {
            "Comet": 312,
            "Dancer": 689
        }

        actual = day._calculate_reindeer_points_after_seconds(
            1_000,
            [
                "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
                "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            ]
        )

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = 1084

        actual = day.calculate(part=2)

        self.assertEqual(expected, actual)
