package twentytwentytwo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay05(t *testing.T) {
	x := assert.New(t)

	t.Run("day 5 part 1 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_5_1_sample.txt")
		expected := "CMZ"

		actual := day05Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 5 part 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_5_1.txt")
		expected := "CNSZFDVLJ"

		actual := day05Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 5 part 2 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_5_2_sample.txt")
		expected := "MCD"

		actual := day05Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 5 part 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_5_2.txt")
		expected := "QNDWLMGNS"

		actual := day05Part02(input)

		x.Equal(expected, actual)
	})
}
