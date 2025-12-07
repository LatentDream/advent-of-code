package main

import (
	"fmt"
	"log"
	"os"

	"aoc/days"
)

func main() {
	fmt.Println("Advent of Code 2025")
	content, err := os.ReadFile("input/day6.txt")
	if err != nil {
		log.Fatal(err)
	}
	days.Day6(string(content))
}
