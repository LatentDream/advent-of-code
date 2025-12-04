package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Advent of Code 2025")
	content, err := os.ReadFile("input/day3.txt")
	if err != nil {
		log.Fatal(err)
	}
	day3(string(content))
}

func day3(content string) {
	input := strings.Split(content, "\n")

	totalPart1 := part1(input)
	fmt.Printf("Part 1: %d", totalPart1)
}
// ANSI color codes for gradient effect
func getColorCode(digit int) string {
	colors := []string{
		"\033[90m", // 0 - dark gray
		"\033[94m", // 1 - dark blue
		"\033[96m", // 2 - cyan
		"\033[92m", // 3 - green
		"\033[93m", // 4 - yellow
		"\033[33m", // 5 - dark yellow
		"\033[91m", // 6 - light red
		"\033[31m", // 7 - red
		"\033[95m", // 8 - magenta
		"\033[97m", // 9 - bright white
	}
	return colors[digit]
}

func resetColor() string {
	return "\033[0m"
}

func printColoredBank(bank string) {
	fmt.Print("Bank: ")
	for _, char := range bank {
		digit, _ := strconv.Atoi(string(char))
		fmt.Printf("%s%c%s", getColorCode(digit), char, resetColor())
	}
	fmt.Println()
}

func part1(banks []string) int {
	total := 0
	for _, bank := range banks {
		if len(bank) == 0 {
			continue
		}
		first, _ := strconv.Atoi(string(bank[0]))
		second, _ := strconv.Atoi(string(bank[1]))

		for i, batterie := range bank {
			if i < 1 {
				continue
			}

			batterieInt, _ := strconv.Atoi(string(batterie))
			if batterieInt > first && i <= len(bank)-2 {
				first = batterieInt
				batterieNextInt, _ := strconv.Atoi(string(bank[i+1]))
				second = batterieNextInt
				continue
			}

			if batterieInt > second {
				second = batterieInt
			}
		}
		joltage := fmt.Sprintf("%d%d", first, second)
		joltageInt, _ := strconv.Atoi(string(joltage))

		fmt.Println("joltage: ", joltage)
		total += int(joltageInt)

	}
	return total
}
