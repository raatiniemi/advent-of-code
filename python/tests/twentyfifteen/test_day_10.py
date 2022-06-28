import unittest
from python.src.twentyfifteen import day_10


class TwentyFifteenDayTenTestCase(unittest.TestCase):
    def test_part_one_example_one(self):
        expected = '11'

        actual = day_10.calculate_input("1")

        self.assertEqual(expected, actual)

    def test_part_one_example_two(self):
        expected = '21'

        actual = day_10.calculate_input("11")

        self.assertEqual(expected, actual)

    def test_part_one_example_three(self):
        expected = '1211'

        actual = day_10.calculate_input("21")

        self.assertEqual(expected, actual)

    def test_part_one_example_four(self):
        expected = '111221'

        actual = day_10.calculate_input("1211")

        self.assertEqual(expected, actual)

    def test_part_one_example_five(self):
        expected = '312211'

        actual = day_10.calculate_input("111221")

        self.assertEqual(expected, actual)

    def test_part_one(self):
        expected = 329356

        actual = day_10.day_ten(part=1)

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = 4666278

        actual = day_10.day_ten(part=2)

        self.assertEqual(expected, actual)
