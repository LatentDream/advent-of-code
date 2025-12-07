package days

import (
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"
)

func Day6(content string) {
	inputs := day6part1Parse(content)

	totalPart1 := calculate(inputs)
	fmt.Printf("Part 1: %d\n", totalPart1)

	inputs = day6part2Parse(content)

	totalPart2 := calculate(inputs)
	fmt.Printf("Part 2: %d\n", totalPart2)
}

type Operator = int

const (
	mult Operator = iota
	plus
)

type Input struct {
	number   []int
	operator Operator
}

func day6part1Parse(content string) []Input {
	inputs := []Input{}

	lines := filterEmpty(strings.Split(content, "\n"))
	nbNumber := len(lines) - 2

	for lineNb, line := range lines {
		line = strings.Trim(line, " ")
		re := regexp.MustCompile(`\s+`)
		line = re.ReplaceAllString(line, " ")
		numbersStr := strings.Split(line, " ")
		for i, numberStr := range numbersStr {
			number, err := strconv.Atoi(numberStr)
			if err != nil {
				log.Fatalf("Failed to convert to int: %s", err)
			}
			if lineNb == 0 {
				inputs = append(inputs, Input{
					number: []int{number},
				})
			} else {
				inputs[i].number = append(inputs[i].number, number)
			}
		}
		if lineNb == nbNumber {
			break
		}
	}

	opLine := lines[nbNumber+1]
	parseOps(opLine, inputs)

	return inputs
}

func day6part2Parse(content string) []Input {
	inputs := []Input{}

	lines := filterEmpty(strings.Split(content, "\n"))
	nbNumber := len(lines) - 2
	firstLine := lines[0]
	inputIdx := 0

	for i, char := range firstLine {
		// Find first number
		j := 0
		for char == ' ' && j < nbNumber {
			j++
			char = rune(lines[j][i])
		}
		if char == ' ' {
			inputIdx++
			continue
		}

		// Build number until reach the end
		numberChars := []rune{}
		for j <= nbNumber && char != ' ' {
			numberChars = append(numberChars, char)
			char = rune(lines[j+1][i])
			j++
		}

		if len(inputs) <= inputIdx {
			inputs = append(inputs, Input{number: []int{}})
		}
		number, err := strconv.Atoi(string(numberChars))
		if err != nil {
			log.Fatalf("Failed to convert str to int %s", err)
		}
		inputs[inputIdx].number = append(inputs[inputIdx].number, number)
	}

	opLine := lines[nbNumber+1]
	parseOps(opLine, inputs)

	return inputs
}

func parseOps(line string, inputs []Input) {
	opLine := strings.Trim(line, " ")
	re := regexp.MustCompile(`\s+`)
	opLine = re.ReplaceAllString(opLine, " ")
	for i, op := range strings.Split(opLine, " ") {
		if op == "*" {
			inputs[i].operator = mult
		} else {
			inputs[i].operator = plus
		}
	}
}

func applyOperator(inputs []int, op Operator) int {
	total := 0
	if op == mult {
		total = 1
	}
	for _, input := range inputs {
		switch op {
		case mult:
			total *= input
		case plus:
			total += input
		}
	}
	return total
}

func printOperation(numbers []int, op Operator, result int) {
	opSymbol := getOperatorSymbol(op)

	expr := fmt.Sprintf("%d", numbers[0])
	for i := 1; i < len(numbers); i++ {
		expr += fmt.Sprintf(" %s %d", opSymbol, numbers[i])
	}

	fmt.Printf("%s = %d\n", expr, result)
}

func getOperatorSymbol(op Operator) string {
	switch op {
	case plus:
		return "+"
	case mult:
		return "*"
	default:
		return "?"
	}
}

func calculate(inputs []Input) int {
	total := 0
	for _, input := range inputs {
		eq := applyOperator(input.number, input.operator)
		total += eq
	}
	return total
}

func day6part2(input []Input) int {
	return -1
}
