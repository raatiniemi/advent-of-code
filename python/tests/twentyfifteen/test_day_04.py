import unittest
from python.src.twentyfifteen import day_04


class TwentyFifteenDayFourTestCase(unittest.TestCase):
    def test_day_four_calculate_part_one_example_one(self):
        expected = 609043

        actual = day_04.day_four_calculate_part_one("abcdef")

        self.assertEqual(expected, actual)

    def test_day_four_calculate_part_one_example_two(self):
        expected = 1048970

        actual = day_04.day_four_calculate_part_one("pqrstuv")

        self.assertEqual(expected, actual)

    def test_day_four_part_one(self):
        expected = 117946

        actual = day_04.day_four(part=1)

        self.assertEqual(expected, actual)

    def test_day_four_part_two(self):
        expected = 3938038

        actual = day_04.day_four(part=2)

        self.assertEqual(expected, actual)
