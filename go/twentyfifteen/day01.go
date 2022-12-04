package twentyfifteen

import "strings"

func day01Part01(input string) int {
	return strings.Count(input, "(") - strings.Count(input, ")")
}

func day01Part02(input string) int {
	value := 0
	for i, character := range input {
		character := string(character)
		if character == "(" {
			value++
		} else {
			value--
		}

		if value == -1 {
			return i + 1
		}
	}
	return value
}
