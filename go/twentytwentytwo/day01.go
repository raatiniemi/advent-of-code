package main

import (
	"sort"
	"strconv"
	"strings"
)

func day01Part01(input string) int {
	caloriesByElf := calculateCaloriesByElf(input)
	return sum(caloriesByElf[len(caloriesByElf)-1:])
}

func day01Part02(input string) int {
	caloriesByElf := calculateCaloriesByElf(input)
	return sum(caloriesByElf[len(caloriesByElf)-3:])
}

func calculateCaloriesByElf(input string) []int {
	var totalCaloriesByElf []int
	groups := strings.Split(strings.Trim(input, "\n"), "\n\n")
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
