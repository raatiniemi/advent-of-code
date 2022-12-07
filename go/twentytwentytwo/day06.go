package twentytwentytwo

import "strings"

func day06Part01(input string) int {
	result := 0
	for _, chars := range window[string](characters(input), 4) {
		s := make(map[string]bool)
		for _, char := range chars {
			s[char] = true
		}
		if len(chars) == len(s) {
			result = strings.Index(input, strings.Join(chars, "")) + len(s)
			break
		}
	}
	return result
}

func day06Part02(input string) int {
	result := 0

	for _, chars := range window[string](characters(input), 14) {
		s := make(map[string]bool)
		for _, char := range chars {
			s[char] = true
		}
		if len(chars) == len(s) {
			result = strings.Index(input, strings.Join(chars, "")) + len(s)
			break
		}
	}
	return result
}
