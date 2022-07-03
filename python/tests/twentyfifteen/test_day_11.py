import unittest
from python.src.twentyfifteen import day_11


class TwentyFifteenDayElevenTestCase(unittest.TestCase):
    def test_part_one_example_one(self):
        expected = False

        actual = day_11._is_valid_part_one("hijklmmn")

        self.assertEqual(expected, actual)

    def test_part_one_example_two(self):
        expected = False

        actual = day_11._is_valid_part_one("abbceffg")

        self.assertEqual(expected, actual)

    def test_part_one_example_three(self):
        expected = False

        actual = day_11._is_valid_part_one("abbcegjk")

        self.assertEqual(expected, actual)

    def test_part_one_example_four(self):
        expected = "abcdffaa"

        actual = day_11._day_eleven_calculate_part_one("abcdefgh")

        self.assertEqual(expected, actual)

    def test_part_one_example_five(self):
        expected = "ghjaabcc"

        actual = day_11._day_eleven_calculate_part_one("ghijklmn")

        self.assertEqual(expected, actual)

    def test_part_one(self):
        expected = "hepxxyzz"

        actual = day_11.day_eleven(part=1)

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = "heqaabcc"

        actual = day_11.day_eleven(part=2)

        self.assertEqual(expected, actual)
