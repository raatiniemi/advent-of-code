package twentytwentytwo

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestDay07(t *testing.T) {
	x := assert.New(t)

	t.Run("day 7 part 1 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_7_1_sample.txt")
		expected := 95437

		actual := day07Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 7 part 1", func(t *testing.T) {
		input := readFile("../../input/2022/day_7_1.txt")
		expected := 1743217

		actual := day07Part01(input)

		x.Equal(expected, actual)
	})

	t.Run("day 7 part 2 sample", func(t *testing.T) {
		input := readFile("../../input/2022/day_7_2_sample.txt")
		expected := 24933642

		actual := day07Part02(input)

		x.Equal(expected, actual)
	})

	t.Run("day 7 part 2", func(t *testing.T) {
		input := readFile("../../input/2022/day_7_2.txt")
		expected := 8319096

		actual := day07Part02(input)

		x.Equal(expected, actual)
	})
}
