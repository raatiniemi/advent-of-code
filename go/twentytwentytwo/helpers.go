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

type Number interface {
	int | int64 | float32 | float64
}

func min[T Number](lhs T, rhs T) T {
	if lhs < rhs {
		return lhs
	}
	return rhs
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

func window[T any](values []T, size int) [][]T {
	var result [][]T
	length := len(values)
	for i := 0; i < length; i++ {
		var items []T
		for _, item := range values[i:min(i+size, length)] {
			items = append(items, item)
		}
		result = append(result, items)
	}
	return result
}
