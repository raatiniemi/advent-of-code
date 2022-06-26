import unittest
from python.src.twentyfifteen import day_02


class TwentyFifteenDayTwoTestCase(unittest.TestCase):
    def test_day_two_calculate_part_one_example_one(self):
        expected = 58

        actual = day_02.day_two_calculate_part_one("2x3x4")

        self.assertEqual(expected, actual)

    def test_day_two_calculate_part_one_example_two(self):
        expected = 43

        actual = day_02.day_two_calculate_part_one("1x1x10")

        self.assertEqual(expected, actual)

    def test_day_two_part_one(self):
        expected = 1606483

        actual = day_02.day_two(part=1)

        self.assertEqual(expected, actual)

    def test_day_two_calculate_part_two_example_one(self):
        expected = 34

        actual = day_02.day_two_calculate_part_two("2x3x4")

        self.assertEqual(expected, actual)

    def test_day_two_calculate_part_two_example_two(self):
        expected = 14

        actual = day_02.day_two_calculate_part_two("1x1x10")

        self.assertEqual(expected, actual)

    def test_day_two_part_two(self):
        expected = 3842356

        actual = day_02.day_two(part=2)

        self.assertEqual(expected, actual)
