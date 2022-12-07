package twentytwentytwo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay06(t *testing.T) {
	x := assert.New(t)

	t.Run("day 6 part 1 sample 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_1_sample_1.txt")
		expected := 7

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_1_sample_2.txt")
		expected := 5

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 3", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_1_sample_3.txt")
		expected := 6

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 4", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_1_sample_4.txt")
		expected := 10

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 5", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_1_sample_5.txt")
		expected := 11

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_1.txt")
		expected := 1480

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_2_sample_1.txt")
		expected := 19

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_2_sample_2.txt")
		expected := 23

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 3", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_2_sample_3.txt")
		expected := 23

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 4", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_2_sample_4.txt")
		expected := 29

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 5", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_2_sample_5.txt")
		expected := 26

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_6_2.txt")
		expected := 2746

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})
}
