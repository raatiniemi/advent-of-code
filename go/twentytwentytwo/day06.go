package twentytwentytwo

import "strings"

func day06Part01(input string) int {
	return findStartOfPacketMarker(input, 4)
}

func findStartOfPacketMarker(input string, size int) int {
	result := 0

	for _, chars := range window[string](characters(input), size) {
		s := make(map[string]bool)
		for _, char := range chars {
			s[char] = true
		}
		if size == len(s) {
			result = strings.Index(input, strings.Join(chars, "")) + size
			break
		}
	}

	return result
}

func day06Part02(input string) int {
	return findStartOfPacketMarker(input, 14)
}
