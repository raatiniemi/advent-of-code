package twentytwentytwo

import "strings"

func day06Part01(input string) int {
	result := 0
	var characters []string
	for _, char := range input {
		characters = append(characters, string(char))
	}

	for _, chars := range window[string](characters, 4) {
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
	var characters []string
	for _, char := range input {
		characters = append(characters, string(char))
	}

	for _, chars := range window[string](characters, 14) {
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
