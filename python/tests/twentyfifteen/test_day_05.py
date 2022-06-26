import unittest
from python.src.twentyfifteen import day_05


class TwentyFifteenDayFiveTestCase(unittest.TestCase):
    def test_day_five_calculate_part_one_example_one(self):
        expected = True

        actual = day_05.day_five_is_string_nice_part_one("ugknbfddgicrmopn")

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_one_example_two(self):
        expected = True

        actual = day_05.day_five_is_string_nice_part_one("aaa")

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_one_example_three(self):
        expected = False

        actual = day_05.day_five_is_string_nice_part_one("jchzalrnumimnmhp")

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_one_example_four(self):
        expected = False

        actual = day_05.day_five_is_string_nice_part_one("haegwjzuvuyypxyu")

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_one_example_five(self):
        expected = False

        actual = day_05.day_five_is_string_nice_part_one("dvszwmarrgswjxmb")

        self.assertEqual(expected, actual)

    def test_day_five_part_one(self):
        expected = 238

        actual = day_05.day_five(part=1)

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_two_example_one(self):
        expected = True

        actual = day_05.day_five_is_string_nice_part_two("qjhvhtzxzqqjkmpb")

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_two_example_two(self):
        expected = True

        actual = day_05.day_five_is_string_nice_part_two("xxyxx")

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_two_example_three(self):
        expected = False

        actual = day_05.day_five_is_string_nice_part_two("uurcxstgmygtbstg")

        self.assertEqual(expected, actual)

    def test_day_five_calculate_part_two_example_four(self):
        expected = False

        actual = day_05.day_five_is_string_nice_part_two("ieodomkazucvgmuy")

        self.assertEqual(expected, actual)

    def test_day_five_part_two(self):
        expected = 69

        actual = day_05.day_five(part=2)

        self.assertEqual(expected, actual)
