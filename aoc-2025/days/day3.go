package days

import (
	"fmt"
	"strconv"
	"strings"
)

func Day3(content string) {
	input := strings.Split(content, "\n")

	totalPart1 := day3part1(input)
	fmt.Printf("Part 1: %d", totalPart1)

	totalPart2 := day3part2(input)
	fmt.Printf("Part 2: %d", totalPart2)
}

func day3part1(banks []string) int {
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

func day3part2(banks []string) int {
	total := 0
	for _, bank := range banks {
		if len(bank) == 0 {
			continue
		}
		digit1, _ := strconv.Atoi(string(bank[0]))
		digit2, _ := strconv.Atoi(string(bank[1]))
		digit3, _ := strconv.Atoi(string(bank[2]))
		digit4, _ := strconv.Atoi(string(bank[3]))
		digit5, _ := strconv.Atoi(string(bank[4]))
		digit6, _ := strconv.Atoi(string(bank[5]))
		digit7, _ := strconv.Atoi(string(bank[6]))
		digit8, _ := strconv.Atoi(string(bank[7]))
		digit9, _ := strconv.Atoi(string(bank[8]))
		digit10, _ := strconv.Atoi(string(bank[9]))
		digit11, _ := strconv.Atoi(string(bank[10]))
		digit12, _ := strconv.Atoi(string(bank[11]))

		// Track positions where each digit was found
		pos1, pos2, pos3, pos4, pos5, pos6 := 0, 1, 2, 3, 4, 5
		pos7, pos8, pos9, pos10, pos11 := 6, 7, 8, 9, 10

		for i, batterie := range bank {
			if i < 1 {
				continue
			}

			batterieInt, _ := strconv.Atoi(string(batterie))
			
			if batterieInt > digit1 && i <= len(bank)-12 {
				digit1, _ = strconv.Atoi(string(bank[i]))
				digit2, _ = strconv.Atoi(string(bank[i+1]))
				digit3, _ = strconv.Atoi(string(bank[i+2]))
				digit4, _ = strconv.Atoi(string(bank[i+3]))
				digit5, _ = strconv.Atoi(string(bank[i+4]))
				digit6, _ = strconv.Atoi(string(bank[i+5]))
				digit7, _ = strconv.Atoi(string(bank[i+6]))
				digit8, _ = strconv.Atoi(string(bank[i+7]))
				digit9, _ = strconv.Atoi(string(bank[i+8]))
				digit10, _ = strconv.Atoi(string(bank[i+9]))
				digit11, _ = strconv.Atoi(string(bank[i+10]))
				digit12, _ = strconv.Atoi(string(bank[i+11]))
				pos1, pos2, pos3, pos4, pos5, pos6 = i, i+1, i+2, i+3, i+4, i+5
				pos7, pos8, pos9, pos10, pos11 = i+6, i+7, i+8, i+9, i+10
				continue
			}

			if batterieInt > digit2 && i <= len(bank)-11 && i > pos1 {
				digit2, _ = strconv.Atoi(string(bank[i]))
				digit3, _ = strconv.Atoi(string(bank[i+1]))
				digit4, _ = strconv.Atoi(string(bank[i+2]))
				digit5, _ = strconv.Atoi(string(bank[i+3]))
				digit6, _ = strconv.Atoi(string(bank[i+4]))
				digit7, _ = strconv.Atoi(string(bank[i+5]))
				digit8, _ = strconv.Atoi(string(bank[i+6]))
				digit9, _ = strconv.Atoi(string(bank[i+7]))
				digit10, _ = strconv.Atoi(string(bank[i+8]))
				digit11, _ = strconv.Atoi(string(bank[i+9]))
				digit12, _ = strconv.Atoi(string(bank[i+10]))
				pos2, pos3, pos4, pos5, pos6 = i, i+1, i+2, i+3, i+4
				pos7, pos8, pos9, pos10, pos11 = i+5, i+6, i+7, i+8, i+9
				continue
			}

			if batterieInt > digit3 && i <= len(bank)-10 && i > pos2 {
				digit3, _ = strconv.Atoi(string(bank[i]))
				digit4, _ = strconv.Atoi(string(bank[i+1]))
				digit5, _ = strconv.Atoi(string(bank[i+2]))
				digit6, _ = strconv.Atoi(string(bank[i+3]))
				digit7, _ = strconv.Atoi(string(bank[i+4]))
				digit8, _ = strconv.Atoi(string(bank[i+5]))
				digit9, _ = strconv.Atoi(string(bank[i+6]))
				digit10, _ = strconv.Atoi(string(bank[i+7]))
				digit11, _ = strconv.Atoi(string(bank[i+8]))
				digit12, _ = strconv.Atoi(string(bank[i+9]))
				pos3, pos4, pos5, pos6 = i, i+1, i+2, i+3
				pos7, pos8, pos9, pos10, pos11 = i+4, i+5, i+6, i+7, i+8
				continue
			}

			if batterieInt > digit4 && i <= len(bank)-9 && i > pos3 {
				digit4, _ = strconv.Atoi(string(bank[i]))
				digit5, _ = strconv.Atoi(string(bank[i+1]))
				digit6, _ = strconv.Atoi(string(bank[i+2]))
				digit7, _ = strconv.Atoi(string(bank[i+3]))
				digit8, _ = strconv.Atoi(string(bank[i+4]))
				digit9, _ = strconv.Atoi(string(bank[i+5]))
				digit10, _ = strconv.Atoi(string(bank[i+6]))
				digit11, _ = strconv.Atoi(string(bank[i+7]))
				digit12, _ = strconv.Atoi(string(bank[i+8]))
				pos4, pos5, pos6, pos7 = i, i+1, i+2, i+3
				pos8, pos9, pos10, pos11 = i+4, i+5, i+6, i+7
				continue
			}

			if batterieInt > digit5 && i <= len(bank)-8 && i > pos4 {
				digit5, _ = strconv.Atoi(string(bank[i]))
				digit6, _ = strconv.Atoi(string(bank[i+1]))
				digit7, _ = strconv.Atoi(string(bank[i+2]))
				digit8, _ = strconv.Atoi(string(bank[i+3]))
				digit9, _ = strconv.Atoi(string(bank[i+4]))
				digit10, _ = strconv.Atoi(string(bank[i+5]))
				digit11, _ = strconv.Atoi(string(bank[i+6]))
				digit12, _ = strconv.Atoi(string(bank[i+7]))
				pos5, pos6, pos7, pos8 = i, i+1, i+2, i+3
				pos9, pos10, pos11 = i+4, i+5, i+6
				continue
			}

			if batterieInt > digit6 && i <= len(bank)-7 && i > pos5 {
				digit6, _ = strconv.Atoi(string(bank[i]))
				digit7, _ = strconv.Atoi(string(bank[i+1]))
				digit8, _ = strconv.Atoi(string(bank[i+2]))
				digit9, _ = strconv.Atoi(string(bank[i+3]))
				digit10, _ = strconv.Atoi(string(bank[i+4]))
				digit11, _ = strconv.Atoi(string(bank[i+5]))
				digit12, _ = strconv.Atoi(string(bank[i+6]))
				pos6, pos7, pos8, pos9 = i, i+1, i+2, i+3
				pos10, pos11 = i+4, i+5
				continue
			}

			if batterieInt > digit7 && i <= len(bank)-6 && i > pos6 {
				digit7, _ = strconv.Atoi(string(bank[i]))
				digit8, _ = strconv.Atoi(string(bank[i+1]))
				digit9, _ = strconv.Atoi(string(bank[i+2]))
				digit10, _ = strconv.Atoi(string(bank[i+3]))
				digit11, _ = strconv.Atoi(string(bank[i+4]))
				digit12, _ = strconv.Atoi(string(bank[i+5]))
				pos7, pos8, pos9 = i, i+1, i+2
				pos10, pos11 = i+3, i+4
				continue
			}

			if batterieInt > digit8 && i <= len(bank)-5 && i > pos7 {
				digit8, _ = strconv.Atoi(string(bank[i]))
				digit9, _ = strconv.Atoi(string(bank[i+1]))
				digit10, _ = strconv.Atoi(string(bank[i+2]))
				digit11, _ = strconv.Atoi(string(bank[i+3]))
				digit12, _ = strconv.Atoi(string(bank[i+4]))
				pos8, pos9, pos10, pos11 = i, i+1, i+2, i+3
				continue
			}

			if batterieInt > digit9 && i <= len(bank)-4 && i > pos8 {
				digit9, _ = strconv.Atoi(string(bank[i]))
				digit10, _ = strconv.Atoi(string(bank[i+1]))
				digit11, _ = strconv.Atoi(string(bank[i+2]))
				digit12, _ = strconv.Atoi(string(bank[i+3]))
				pos9, pos10, pos11 = i, i+1, i+2
				continue
			}

			if batterieInt > digit10 && i <= len(bank)-3 && i > pos9 {
				digit10, _ = strconv.Atoi(string(bank[i]))
				digit11, _ = strconv.Atoi(string(bank[i+1]))
				digit12, _ = strconv.Atoi(string(bank[i+2]))
				pos10, pos11 = i, i+1
				continue
			}

			if batterieInt > digit11 && i <= len(bank)-2 && i > pos10 {
				digit11, _ = strconv.Atoi(string(bank[i]))
				digit12, _ = strconv.Atoi(string(bank[i+1]))
				pos11 = i
				continue
			}

			if batterieInt > digit12 && i > pos11 {
				digit12 = batterieInt
			}
		}
		
		joltage := fmt.Sprintf("%d%d%d%d%d%d%d%d%d%d%d%d", digit1, digit2, digit3, digit4, digit5, digit6, digit7, digit8, digit9, digit10, digit11, digit12)
		joltageInt, _ := strconv.Atoi(joltage)

		fmt.Println("joltage: ", joltage)
		total += joltageInt
	}
	return total
}
