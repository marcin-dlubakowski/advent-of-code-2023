package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type game struct {
	pulls []pull
	id    int
}

type pull struct {
	id      int
	r, g, b int
}

func parseColorData(line string, color string) int {
	match := regexp.MustCompile("(\\d+) " + color).FindStringSubmatch(line)
	if len(match) > 0 {
		val, _ := strconv.Atoi(match[1])
		return val
	}
	return 0
}

func isGamePossible(gameData game, r int, g int, b int) bool {
	for _, pullData := range gameData.pulls {
		if pullData.r > r || pullData.g > g || pullData.b > b || (pullData.r+pullData.g+pullData.b) > (r+g+b) {
			return false
		}
	}
	return true
}

func parseGameData(line string) game {
	pulls := []pull{}
	// Parse game id
	gameIdMatch := regexp.MustCompile(`Game (\d+)`).FindStringSubmatch(line)
	gameId, _ := strconv.Atoi(gameIdMatch[1])
	// Parse pulls
	pullsData := strings.Split(strings.Split(line, ": ")[1], "; ")
	for id, pullData := range pullsData {
		r := parseColorData(pullData, "red")
		g := parseColorData(pullData, "green")
		b := parseColorData(pullData, "blue")
		pulls = append(pulls, pull{id: id, r: r, g: g, b: b})
	}
	return game{id: gameId, pulls: pulls}
}

func main() {
	data, err := os.ReadFile("input.txt")
	check(err)
	lines := strings.Split(string(data), "\n")

	result := 0

	for _, line := range lines {
		if line == "" {
			continue
		}

		gameData := parseGameData(line)
		possible := isGamePossible(gameData, 12, 13, 14)
		fmt.Println("Game #", gameData.id, "possible =", possible)
		if possible {
			result += gameData.id
		}
	}
	fmt.Println("Result = ", result)
}
