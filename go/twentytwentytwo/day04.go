package main

import (
	"strconv"
	"strings"
)

func day04Part01(input string) int {
	overlaps := 0
	lines := readLines(input)
	for _, chunk := range chunked(parseRangeBounds(lines), 2) {
		lhs := chunk[0]
		rhs := chunk[1]
		if isBetween(lhs.lower, rhs) && isBetween(lhs.upper, rhs) {
			overlaps++
		} else if isBetween(rhs.lower, lhs) && isBetween(rhs.upper, lhs) {
			overlaps++
		}
	}
	return overlaps
}

func isBetween(value int, rangeBound RangeBound) bool {
	return value >= rangeBound.lower && value <= rangeBound.upper
}

func parseRangeBounds(values []string) []RangeBound {
	var result []RangeBound
	for _, group := range values {
		bounds := strings.Split(group, ",")
		for _, bound := range bounds {
			result = append(result, parseRangeBound(bound))
		}
	}
	return result
}

func parseRangeBound(v string) RangeBound {
	bounds := strings.Split(v, "-")
	lower := parseInt(bounds[0])
	upper := parseInt(bounds[1])
	return RangeBound{lower: lower, upper: upper}
}

func parseInt(v string) int {
	x, err := strconv.Atoi(v)
	if err != nil {
		return -1
	}
	return x
}

type RangeBound struct {
	lower int
	upper int
}

func day04Part02(input string) int {
	result := 0

	groups := readLines(input)
	for _, rangeBounds := range chunked(parseRangeBounds(groups), 2) {
		lhs := rangeBounds[0]
		rhs := rangeBounds[1]
		if isBetween(lhs.lower, rhs) || isBetween(lhs.upper, rhs) {
			result++
		} else if isBetween(rhs.lower, lhs) || isBetween(rhs.upper, lhs) {
			result++
		}
	}
	return result
}
