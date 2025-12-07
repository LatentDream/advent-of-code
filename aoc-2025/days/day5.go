package days

import (
	"fmt"
	"log"
	"slices"
	"strconv"
	"strings"
)

func Day5(content string) {
	ranges, idx := split(strings.Split(content, "\n"))

	totalPart1 := day5part1(ranges, idx)
	fmt.Printf("Part 1: %d\n", totalPart1)

	totalPart2 := day5part2(ranges)
	fmt.Printf("Part 2: %d\n", totalPart2)
}

type Range struct {
	start int
	end   int
}

func (r *Range) isIn(value int) bool {
	return value >= r.start && value <= r.end
}

func split(lines []string) ([]Range, []int) {
	ranges := []Range{}
	idx := []int{}

	delimeter := 0
	for i, line := range lines {
		if len(line) == 0 {
			delimeter = i + 1
			break
		}

		parts := strings.Split(line, "-")
		start, err := strconv.Atoi(parts[0])
		if err != nil {
			log.Fatalf("Failed to parse int: %s", err)
		}
		end, err := strconv.Atoi(parts[1])
		if err != nil {
			log.Fatalf("Failed to parse int: %s", err)
		}

		ranges = append(ranges, Range{
			start: start,
			end:   end,
		})
	}

	for _, line := range lines[delimeter:] {
		if len(line) == 0 {
			break
		}

		id, err := strconv.Atoi(line)
		if err != nil {
			log.Fatalf("Failed to parse int: %s", err)
		}
		idx = append(idx, id)
	}

	return ranges, idx
}

func day5part1(ranges []Range, idx []int) int {
	nbValid := 0
	for _, id := range idx {
		for _, ran := range ranges {
			if ran.isIn(id) {
				nbValid++
				break
			}
		}
	}

	return nbValid
}

func day5part2(ranges []Range) int {
	// Sort
	slices.SortFunc(ranges, func(a Range, b Range) int { return a.start - b.start })

	// Merge
	merged := []Range{ranges[0]}
	last := 0

	for _, ran := range ranges[0:] {
		if merged[last].isIn(ran.start) && ran.end > merged[last].end {
			merged[last].end = ran.end
		} else if !merged[last].isIn(ran.start) {
			merged = append(merged, ran)
			last++
		}
	}

	// Calculate
	nbValid := 0
	for _, ran := range merged {
		nbValid += ran.end - ran.start + 1
	}

	return nbValid
}
