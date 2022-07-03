import unittest
from python.src.twentyfifteen import day_12


class TwentyFifteenDayTwelveTestCase(unittest.TestCase):
    def test_part_one_example_one(self):
        expected = 6

        actual = day_12._day_twelve_calculate_part_one("[1,2,3]")

        self.assertEqual(expected, actual)

    def test_part_one_example_two(self):
        expected = 6

        actual = day_12._day_twelve_calculate_part_one('{"a":2,"b":4}')

        self.assertEqual(expected, actual)

    def test_part_one_example_three(self):
        expected = 3

        actual = day_12._day_twelve_calculate_part_one('[[[3]]]')

        self.assertEqual(expected, actual)

    def test_part_one_example_four(self):
        expected = 3

        actual = day_12._day_twelve_calculate_part_one('{"a":{"b":4},"c":-1}')

        self.assertEqual(expected, actual)

    def test_part_one_example_five(self):
        expected = 0

        actual = day_12._day_twelve_calculate_part_one('{"a":[-1,1]}')

        self.assertEqual(expected, actual)

    def test_part_one_example_six(self):
        expected = 0

        actual = day_12._day_twelve_calculate_part_one('[-1,{"a":1}]')

        self.assertEqual(expected, actual)

    def test_part_one_example_seven(self):
        expected = 0

        actual = day_12._day_twelve_calculate_part_one("[]")

        self.assertEqual(expected, actual)

    def test_part_one_example_eight(self):
        expected = 0

        actual = day_12._day_twelve_calculate_part_one("{}")

        self.assertEqual(expected, actual)

    def test_part_one(self):
        expected = 156366

        actual = day_12.day_twelve(part=1)

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = 96852

        actual = day_12.day_twelve(part=2)

        self.assertEqual(expected, actual)
