package main

import (
	"sort"
	"strconv"
	"strings"
)

func part1(input string) int {
	caloriesByElf := calculateCaloriesByElf(input)
	return sum(caloriesByElf[len(caloriesByElf)-1:])
}

func part2(input string) int {
	caloriesByElf := calculateCaloriesByElf(input)
	return sum(caloriesByElf[len(caloriesByElf)-3:])
}

func calculateCaloriesByElf(input string) []int {
	var totalCaloriesByElf []int
	groups := strings.Split(input, "\n\n")
	for _, group := range groups {
		total := 0
		items := strings.Split(group, "\n")
		for _, item := range items {
			calories, err := strconv.Atoi(item)
			if err != nil {
				continue
			}

			total += calories
		}
		totalCaloriesByElf = append(totalCaloriesByElf, total)
	}
	sort.Ints(totalCaloriesByElf)
	return totalCaloriesByElf
}

func sum(values []int) int {
	sum := 0
	for _, value := range values {
		sum += value
	}
	return sum
}
