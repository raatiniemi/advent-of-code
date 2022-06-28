import unittest
from python.src.twentyfifteen import day_09


class TwentyFifteenDayNineTestCase(unittest.TestCase):
    def test_day_nine_calculate_part_one_example_one(self):
        expected = 605

        actual = day_09.day_nine_calculate_part_one(
            "\n".join([
                "London to Dublin = 464",
                "London to Belfast = 518",
                "Dublin to Belfast = 141"
            ])
        )

        self.assertEqual(expected, actual)

    def test_day_nine_part_one(self):
        expected = 141

        actual = day_09.day_nine(part=1)

        self.assertEqual(expected, actual)

    def test_day_nine_part_two(self):
        expected = 736

        actual = day_09.day_nine(part=2)

        self.assertEqual(expected, actual)
