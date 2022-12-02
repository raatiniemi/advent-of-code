package main

import (
	"fmt"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay01(t *testing.T) {
	x := assert.New(t)

	t.Run("part 1 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_1_1_sample.txt")
		expected := 24000

		actual := part1(input)

		x.Equal(expected, actual)
	})

	t.Run("part 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_1_1.txt")
		expected := 71934

		actual := part1(input)

		x.Equal(expected, actual)
	})

	t.Run("part 2 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_1_2_sample.txt")
		expected := 45000

		actual := part2(input)

		x.Equal(expected, actual)
	})

	t.Run("part 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_1_2.txt")
		expected := 211447

		actual := part2(input)

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
