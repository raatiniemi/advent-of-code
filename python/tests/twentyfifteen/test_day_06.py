import unittest
from python.src.twentyfifteen import day_06


class TwentyFifteenDaySixTestCase(unittest.TestCase):
    def test_day_six_calculate_part_one_example_one(self):
        expected = 1_000_000

        actual = day_06.day_six_calculate_part_one(
            "turn on 0,0 through 999,999"
        )

        self.assertEqual(expected, actual)

    def test_day_six_calculate_part_one_example_two(self):
        expected = 1_000

        actual = day_06.day_six_calculate_part_one("toggle 0,0 through 999,0")

        self.assertEqual(expected, actual)

    def test_day_six_calculate_part_one_example_three(self):
        expected = 0

        actual = day_06.day_six_calculate_part_one(
            "turn off 499,499 through 500,500"
        )

        self.assertEqual(expected, actual)

    def test_day_six_part_one(self):
        expected = 569999

        actual = day_06.day_six(part=1)

        self.assertEqual(expected, actual)

    def test_day_six_calculate_part_two_example_one(self):
        expected = 1

        actual = day_06.day_six_calculate_part_two("turn on 0,0 through 0,0")

        self.assertEqual(expected, actual)

    def test_day_six_calculate_part_two_example_two(self):
        expected = 2_000_000

        actual = day_06.day_six_calculate_part_two("toggle 0,0 through 999,999")

        self.assertEqual(expected, actual)

    def test_day_six_part_two(self):
        expected = 17836115

        actual = day_06.day_six(part=2)

        self.assertEqual(expected, actual)
