package main

import (
	"strings"
	"unicode"
)

func day03Part01(input string) int {
	var priorities []int

	for _, rucksack := range parseRucksacks(input) {
		characters := make(map[int32]bool)

		splitIndex := len(rucksack) / 2
		first, second := rucksack[:splitIndex], rucksack[splitIndex:]
		for _, character := range first {
			s := string(character)
			if strings.Contains(second, s) {
				characters[character] = true
			}
		}

		for character := range characters {
			priorities = append(priorities, calculatePriority(character))
		}
	}
	return sum(priorities)
}

func parseRucksacks(input string) []string {
	return strings.Split(strings.Trim(input, "\n"), "\n")
}

func calculatePriority(character int32) int {
	var offset int32
	if unicode.IsUpper(character) {
		offset = 38
	} else {
		offset = 96
	}

	return int(character - offset)
}

func day03Part02(input string) int {
	var badges []int32

	for _, group := range chunked(parseRucksacks(input), 3) {
		var badge int32

		for _, character := range group[0] {
			if foundInAll(character, group[1:]) {
				badge = character
				break
			}
		}

		badges = append(badges, badge)
	}

	var priorities []int
	for _, character := range badges {
		priorities = append(priorities, calculatePriority(character))
	}
	return sum(priorities)
}

func foundInAll(x int32, values []string) bool {
	occurrences := 0
	for _, value := range values {
		if strings.Contains(value, string(x)) {
			occurrences++
		}
	}
	return occurrences == len(values)
}
