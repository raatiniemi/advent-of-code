package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay01(t *testing.T) {
	x := assert.New(t)

	t.Run("day 1 part 1 sample 1", func(t *testing.T) {
		expected := 0

		actual := day01Part01("(())")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 2", func(t *testing.T) {
		expected := 0

		actual := day01Part01("()()")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 3", func(t *testing.T) {
		expected := 3

		actual := day01Part01("(((")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 4", func(t *testing.T) {
		expected := 3

		actual := day01Part01("(()(()(")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 5", func(t *testing.T) {
		expected := 3

		actual := day01Part01("))(((((")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 6", func(t *testing.T) {
		expected := -1

		actual := day01Part01("())")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 7", func(t *testing.T) {
		expected := -1

		actual := day01Part01("))(")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 8", func(t *testing.T) {
		expected := -3

		actual := day01Part01(")))")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1 sample 9", func(t *testing.T) {
		expected := -3

		actual := day01Part01(")())())")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 1", func(t *testing.T) {
		input := readFile("../../input/2015/day_1_1.txt")
		expected := 74

		actual := day01Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 2 sample 1", func(t *testing.T) {
		expected := 1

		actual := day01Part02(")")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 2 sample 1", func(t *testing.T) {
		expected := 5

		actual := day01Part02("()())")

		x.Equal(expected, actual)
	})

	t.Run("day 1 part 2", func(t *testing.T) {
		input := readFile("../../input/2015/day_1_2.txt")
		expected := 1795

		actual := day01Part02(input)

		x.Equal(expected, actual)
	})
}

func readFile(path string) string {
	bytes, err := os.ReadFile(path)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	return string(bytes)
}
