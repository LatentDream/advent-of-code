package main

import (
	"fmt"
	"log"
	"os"

	"aoc/days"
)

func main() {
	fmt.Println("Advent of Code 2025")
	content, err := os.ReadFile("input/day4.txt")
	if err != nil {
		log.Fatal(err)
	}
	days.Day4(string(content))
}
