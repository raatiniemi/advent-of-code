package main

func sum(values []int) int {
	sum := 0
	for _, value := range values {
		sum += value
	}
	return sum
}

func chunked(values []string, size int) [][]string {
	var result [][]string
	for i := 0; i < len(values); i = i + size {
		var items []string
		for _, item := range values[i : i+size] {
			items = append(items, item)
		}
		result = append(result, items)
	}
	return result
}