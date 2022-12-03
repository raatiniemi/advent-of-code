package main

import (
	"strings"
)

const (
	Rock     = 1
	Paper    = 2
	Scissors = 3
)
const (
	Lose = 1
	Draw = 2
	Win  = 3
)

func calculateScoreFromOutcome(lhs int, rhs int) int {
	if lhs == Rock && rhs == Paper {
		return rhs + 6
	}
	if lhs == Paper && rhs == Scissors {
		return rhs + 6
	}
	if lhs == Scissors && rhs == Rock {
		return rhs + 6
	}
	if lhs == Paper && rhs == Rock {
		return rhs + 0
	}
	if lhs == Rock && rhs == Scissors {
		return rhs + 0
	}
	if lhs == Scissors && rhs == Paper {
		return rhs + 0
	}
	return rhs + 3
}

func moveForOpponent(s string) int {
	switch s {
	case "A":
		return Rock
	case "B":
		return Paper
	default:
		return Scissors
	}
}

func day02Part01(input string) int {
	score := 0
	rounds := strings.Split(strings.Trim(input, "\n"), "\n")
	for _, round := range rounds {
		lhs, rhs := moves(round)
		score += calculateScoreFromOutcome(lhs, rhs)
	}
	return score
}

func moves(game string) (int, int) {
	moves := strings.Split(game, " ")
	return moveForOpponent(moves[0]), mapMoveForMe(moves[1])
}

func mapMoveForMe(s string) int {
	switch s {
	case "X":
		return Rock
	case "Y":
		return Paper
	default:
		return Scissors
	}
}

func day02Part02(input string) int {
	score := 0
	rounds := strings.Split(strings.Trim(input, "\n"), "\n")
	for _, round := range rounds {
		lhs, rhs := opponentMoveWithSuggestedOutcome(round)
		score += calculateScoreFromOutcome(lhs, moveFromOutcome(lhs, rhs))
	}
	return score
}

func opponentMoveWithSuggestedOutcome(game string) (int, int) {
	x := strings.Split(game, " ")
	return moveForOpponent(x[0]), suggestedOutcome(x[1])
}

func suggestedOutcome(s string) int {
	switch s {
	case "X":
		return Lose
	case "Y":
		return Draw
	default:
		return Win
	}
}

func moveFromOutcome(lhs int, rhs int) int {
	if lhs == Rock && rhs == Win {
		return Paper
	}
	if lhs == Paper && rhs == Win {
		return Scissors
	}
	if lhs == Scissors && rhs == Win {
		return Rock
	}
	if lhs == Paper && rhs == Lose {
		return Rock
	}
	if lhs == Rock && rhs == Lose {
		return Scissors
	}
	if lhs == Scissors && rhs == Lose {
		return Paper
	}
	return lhs
}
