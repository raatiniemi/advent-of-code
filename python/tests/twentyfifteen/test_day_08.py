import unittest
from python.src.twentyfifteen import day_08


class TwentyFifteenDayEightTestCase(unittest.TestCase):
    def test_day_eight_part_one(self):
        expected = 1371

        actual = day_08.day_eight(part=1)

        self.assertEqual(expected, actual)

    def test_day_eight_part_two(self):
        expected = 2117

        actual = day_08.day_eight(part=2)

        self.assertEqual(expected, actual)
