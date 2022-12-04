package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay03(t *testing.T) {
	x := assert.New(t)

	t.Run("day 3 part 1 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_3_1_sample.txt")
		expected := 157

		actual := day03Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 3 part 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_3_1.txt")
		expected := 7848

		actual := day03Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 3 part 2 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_3_2_sample.txt")
		expected := 70

		actual := day03Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 3 part 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_3_2.txt")
		expected := 2616

		actual := day03Part02(input)

		x.Equal(expected, actual)
	})
}
