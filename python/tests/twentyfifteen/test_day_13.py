import unittest
from python.src.twentyfifteen import day_13


class TwentyFifteenDayThirteenTestCase(unittest.TestCase):
    def test_part_one_example_one(self):
        expected = 330

        actual = day_13._day_thirteen_calculate_part_one(
            "\n".join([
                "Alice would gain 54 happiness units by sitting next to Bob.",
                "Alice would lose 79 happiness units by sitting next to Carol.",
                "Alice would lose 2 happiness units by sitting next to David.",
                "Bob would gain 83 happiness units by sitting next to Alice.",
                "Bob would lose 7 happiness units by sitting next to Carol.",
                "Bob would lose 63 happiness units by sitting next to David.",
                "Carol would lose 62 happiness units by sitting next to Alice.",
                "Carol would gain 60 happiness units by sitting next to Bob.",
                "Carol would gain 55 happiness units by sitting next to David.",
                "David would gain 46 happiness units by sitting next to Alice.",
                "David would lose 7 happiness units by sitting next to Bob.",
                "David would gain 41 happiness units by sitting next to Carol."
            ])
        )

        self.assertEqual(expected, actual)

    def test_part_one(self):
        expected = 733

        actual = day_13.day_thirteen(part=1)

        self.assertEqual(expected, actual)

    def test_part_two(self):
        expected = 725

        actual = day_13.day_thirteen(part=2)

        self.assertEqual(expected, actual)
