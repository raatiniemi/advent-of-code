package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay02(t *testing.T) {
	x := assert.New(t)

	t.Run("day 2 part 1 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_2_1_sample.txt")
		expected := 15

		actual := day02Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 2 part 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_2_1.txt")
		expected := 11063

		actual := day02Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 2 part 2 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_2_2_sample.txt")
		expected := 12

		actual := day02Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 2 part 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_2_2.txt")
		expected := 10349

		actual := day02Part02(input)

		x.Equal(expected, actual)
	})
}
