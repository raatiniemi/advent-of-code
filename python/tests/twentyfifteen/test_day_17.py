import unittest
from python.src.twentyfifteen import day_17 as day


class TwentyFifteenDaySeventeenTestCase(unittest.TestCase):
    def test_part_one_example_one(self):
        expected = [
            (20, 5),
            (20, 5),
            (15, 10),
            (15, 5, 5)
        ]

        actual = day._calculate_combinations(
            25,
            "\n".join(["20", "15", "10", "5", "5"])
        )

        self.assertEqual(expected, actual)

    def test_part_one(self):
        expected = 1638

        actual = day.calculate(part=1)

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = 17

        actual = day.calculate(part=2)

        self.assertEqual(expected, actual)
