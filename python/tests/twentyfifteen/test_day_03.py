import unittest
from python.src.twentyfifteen import day_03


class TwentyFifteenDayThreeTestCase(unittest.TestCase):
    def test_day_three_calculate_part_one_example_one(self):
        expected = 2

        actual = day_03.day_three_calculate_part_one(">")

        self.assertEqual(expected, actual)

    def test_day_three_calculate_part_one_example_two(self):
        expected = 4

        actual = day_03.day_three_calculate_part_one("^>v<")

        self.assertEqual(expected, actual)

    def test_day_three_calculate_part_one_example_three(self):
        expected = 2

        actual = day_03.day_three_calculate_part_one("^v^v^v^v^v")

        self.assertEqual(expected, actual)

    def test_day_three_part_one(self):
        expected = 2565

        actual = day_03.day_three(part=1)

        self.assertEqual(expected, actual)

    def test_day_three_calculate_part_two_example_one(self):
        expected = 3

        actual = day_03.day_three_calculate_part_two("^v")

        self.assertEqual(expected, actual)

    def test_day_three_calculate_part_two_example_two(self):
        expected = 3

        actual = day_03.day_three_calculate_part_two("^>v<")

        self.assertEqual(expected, actual)

    def test_day_three_calculate_part_two_example_three(self):
        expected = 11

        actual = day_03.day_three_calculate_part_two("^v^v^v^v^v")

        self.assertEqual(expected, actual)

    def test_day_three_part_two(self):
        expected = 2639

        actual = day_03.day_three(part=2)

        self.assertEqual(expected, actual)
