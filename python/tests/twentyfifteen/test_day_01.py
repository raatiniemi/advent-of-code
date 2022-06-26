import unittest
from python.src.twentyfifteen import day_01


class TwentyFifteenDayOneTestCase(unittest.TestCase):
    def test_day_one_calculate_part_one_example_one(self):
        expected = 0

        lhs = day_01.day_one_calculate_part_one("(())")
        rhs = day_01.day_one_calculate_part_one("()()")

        self.assertEqual(expected, lhs)
        self.assertEqual(lhs, rhs)

    def test_day_one_calculate_part_one_example_two(self):
        expected = 3

        lhs = day_01.day_one_calculate_part_one("(((")
        rhs = day_01.day_one_calculate_part_one("(()(()(")

        self.assertEqual(expected, lhs)
        self.assertEqual(lhs, rhs)

    def test_day_one_calculate_part_one_example_three(self):
        expected = 3

        actual = day_01.day_one_calculate_part_one("))(((((")

        self.assertEqual(expected, actual)

    def test_day_one_calculate_part_one_example_four(self):
        expected = -1

        lhs = day_01.day_one_calculate_part_one("())")
        rhs = day_01.day_one_calculate_part_one("))(")

        self.assertEqual(expected, lhs)
        self.assertEqual(lhs, rhs)

    def test_day_one_calculate_part_one_example_five(self):
        expected = -3

        lhs = day_01.day_one_calculate_part_one(")))")
        rhs = day_01.day_one_calculate_part_one(")())())")

        self.assertEqual(expected, lhs)
        self.assertEqual(lhs, rhs)

    def test_day_one_part_one(self):
        expected = 74

        actual = day_01.day_one(part=1)

        self.assertEqual(expected, actual)

    def test_day_one_part_two(self):
        expected = 1795

        actual = day_01.day_one(part=2)

        self.assertEqual(expected, actual)
