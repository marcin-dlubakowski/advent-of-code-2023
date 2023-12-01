package main

import (
	"fmt"
	"os"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

var digits = map[string]int{
	"1":     1,
	"one":   1,
	"2":     2,
	"two":   2,
	"3":     3,
	"three": 3,
	"4":     4,
	"four":  4,
	"5":     5,
	"five":  5,
	"6":     6,
	"six":   6,
	"7":     7,
	"seven": 7,
	"8":     8,
	"eight": 8,
	"9":     9,
	"nine":  9,
}

func main() {
	data, err := os.ReadFile("input.txt")
	check(err)
	lines := strings.Split(string(data), "\n")
	total := 0
	for _, line := range lines {
		fmt.Println(line)
		first := 0
		last := 0
	forward:
		for i := 0; i <= len(line); i++ {
			truncLine := line[i:]
			for k, v := range digits {
				if strings.HasPrefix(truncLine, k) {
					first = v
					break forward
				}
			}
		}
	backward:
		for i := len(line); i >= 0; i-- {
			truncLine := line[0:i]
			for k, v := range digits {
				if strings.HasSuffix(truncLine, k) {
					last = v
					break backward
				}
			}
		}
		fmt.Println("First: ", first, " last: ", last)
		total += first*10 + last // this will evaluate to 0 if there are no digits in line
	}
	fmt.Println("Result is: ", total)
}
