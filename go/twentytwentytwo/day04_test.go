package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay04(t *testing.T) {
	x := assert.New(t)

	t.Run("day 4 part 1 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_4_1_sample.txt")
		expected := 2

		actual := day04Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 4 part 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_4_1.txt")
		expected := 471

		actual := day04Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 4 part 2 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_4_2_sample.txt")
		expected := 4

		actual := day04Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 4 part 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_4_2.txt")
		expected := 888

		actual := day04Part02(input)

		x.Equal(expected, actual)
	})
}
