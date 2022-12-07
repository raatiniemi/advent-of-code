package twentytwentytwo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay06(t *testing.T) {
	x := assert.New(t)

	t.Run("day 6 part 1 sample 1", func(t *testing.T) {
		input := "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
		expected := 7

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 2", func(t *testing.T) {
		input := "bvwbjplbgvbhsrlpgdmjqwftvncz"
		expected := 5

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 3", func(t *testing.T) {
		input := "nppdvjthqldpwncqszvftbrmjlhg"
		expected := 6

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 4", func(t *testing.T) {
		input := "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
		expected := 10

		actual := day06Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 1 sample 5", func(t *testing.T) {
		input := "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
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
		input := "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
		expected := 19

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 2", func(t *testing.T) {
		input := "bvwbjplbgvbhsrlpgdmjqwftvncz"
		expected := 23

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 3", func(t *testing.T) {
		input := "nppdvjthqldpwncqszvftbrmjlhg"
		expected := 23

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 4", func(t *testing.T) {
		input := "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
		expected := 29

		actual := day06Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 6 part 2 sample 5", func(t *testing.T) {
		input := "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"
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
