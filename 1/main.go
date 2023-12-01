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

func main() {
	data, err := os.ReadFile("input.txt")
	check(err)
	lines := strings.Split(string(data), "\n")
	total := 0
	for _, line := range lines {
		fmt.Println(line)
		first := 0
		last := 0
		for _, rune := range line {
			val := int(rune) - 48 // 48 == 0 in ascii
			if val >= 1 && val <= 9 {
				if first == 0 {
					first = val
				}
				last = val
			}
		}
		fmt.Printf("%d, %d\n", first, last)
		total += first*10 + last // this will evaluate to 0 if there are no digits in line
	}
	fmt.Println("Result is: ", total)
}
