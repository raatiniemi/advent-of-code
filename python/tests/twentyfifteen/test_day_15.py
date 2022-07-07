import unittest
from python.src.twentyfifteen import day_15 as day


class TwentyFifteenDayFifteenTestCase(unittest.TestCase):
    def test_part_one(self):
        expected = 21367368

        actual = day.calculate(part=1)

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = 1766400

        actual = day.calculate(part=2)

        self.assertEqual(expected, actual)
