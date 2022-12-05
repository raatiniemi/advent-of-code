package twentytwentytwo

import "strings"

func readLines(input string) []string {
	return strings.Split(strings.Trim(input, "\n"), "\n")
}

func sum(values []int) int {
	sum := 0
	for _, value := range values {
		sum += value
	}
	return sum
}

func chunked[T any](values []T, size int) [][]T {
	var result [][]T
	for i := 0; i < len(values); i = i + size {
		var items []T
		for _, item := range values[i : i+size] {
			items = append(items, item)
		}
		result = append(result, items)
	}
	return result
}
