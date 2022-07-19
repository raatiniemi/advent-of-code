import unittest
from python.src.twentyfifteen import day_16 as day


class TwentyFifteenDaySixteenTestCase(unittest.TestCase):
    def test_part_one(self):
        expected = 213

        actual = day.calculate(part=1)

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = 323

        actual = day.calculate(part=2)

        self.assertEqual(expected, actual)
