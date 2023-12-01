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
	for _, line := range lines {
		fmt.Println(line)
	}
}
