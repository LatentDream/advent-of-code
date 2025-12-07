package days

import (
	"fmt"
	"strings"
)

func Day4(content string) {
	input := filterEmpty(strings.Split(content, "\n"))

	totalPart1 := day4part1(input)
	fmt.Printf("Part 1: %d\n", totalPart1)

	totalPart2 := day4part2(input)
	fmt.Printf("Part 2: %d\n", totalPart2)
}

type move struct {
	dx int
	dy int
}

type position struct {
	x int
	y int
}

var adjacents = []move{
	{-1, -1},
	{0, -1},
	{1, -1},
	{-1, 0},
	{1, 0},
	{-1, 1},
	{-0, 1},
	{1, 1},
}

//   0 1 2 3 4 x
// 0 @ . @ . . @
// 1 . . . . . .
// 2 @ @ . @ @ .
// 4 . . @ @ @ .
// y . . @ @ @ .

func day4part1(input []string) int {
	return len(foundAccessible(input))
}

func foundAccessible(input []string) []position {
	accessible := []position{}

	for y, row := range input {
		for x, el := range row {

			if el != '@' {
				continue
			}

			nbFree := 0
			for _, move := range adjacents {
				if isFree(x+move.dx, y+move.dy, input) {
					nbFree++
				}
			}

			if nbFree >= 5 {
				accessible = append(accessible, position{x, y})
			}
		}
	}
	return accessible
}

func isFree(x, y int, input []string) bool {
	if y < 0 || y >= len(input) {
		return true
	}
	if x < 0 || x >= len(input[0]) {
		return true
	}
	if input[y][x] != '@' {
		return true
	}

	return false
}

func day4part2(input []string) any {
	total := 0
	for {
		accessibles := foundAccessible(input)

		for _, roll := range accessibles {
			row := []byte(input[roll.y])
			row[roll.x] = '.'
			input[roll.y] = string(row)
		}

		total += len(accessibles)
		if len(accessibles) == 0 {
			break
		}
	}
	return total
}
