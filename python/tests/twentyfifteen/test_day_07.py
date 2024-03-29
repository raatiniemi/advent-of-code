import unittest
from python.src.twentyfifteen import day_07


class TwentyFifteenDaySevenTestCase(unittest.TestCase):
    def test_day_seven_calculate_part_one_example(self):
        expected = {
            'd': 72,
            'e': 507,
            'f': 492,
            'g': 114,
            'h': 65412,
            'i': 65079,
            'x': 123,
            'y': 456
        }

        actual = day_07.day_seven_calculate_part_one(
            "\n".join([
                "123 -> x",
                "456 -> y",
                "x AND y -> d",
                "x OR y -> e",
                "x LSHIFT 2 -> f",
                "y RSHIFT 2 -> g",
                "NOT x -> h",
                "NOT y -> i"
            ])
        )

        self.assertEqual(expected, actual)

    def test_day_seven_part_one(self):
        expected = 46065

        actual = day_07.day_seven(part=1)

        self.assertEqual(expected, actual)

    def test_day_seven_part_two(self):
        expected = 14134

        actual = day_07.day_seven(part=2)

        self.assertEqual(expected, actual)
