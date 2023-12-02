package main

import (
	"bufio"
	"fmt"
	"os"
  "regexp"
  "strconv"
  "strings"
)

func part1(game_input string) (int, int, int, int)  {
  var maxFoundBlue int = 0
  var maxFoundGreen int = 0
  var maxFoundRed int = 0

  gameIDRe := regexp.MustCompile(`Game (\d+):`)
  match := gameIDRe.FindStringSubmatch(game_input)[0]
  id := strings.Split(strings.Split(match, " ")[1], ":")[0]
  gameID, err := strconv.Atoi(id)
  if err != nil {
    fmt.Println("Error converting game id to integer:", err)
  }

  re := regexp.MustCompile(`(\d+) (\w+)`)

  input := strings.Split(game_input, ":")[1]
  hands := strings.Split(input, ";")

  for _, hand := range hands {
    colorCounts := make(map[string]int)
    matches := re.FindAllStringSubmatch(hand, -1)

    // Sum the quantities for each color
		for _, match := range matches {
			quantity, err := strconv.Atoi(match[1])
			if err != nil {
				fmt.Println("Error converting quantity to integer:", err)
				continue
			}

			color := match[2]
			colorCounts[color] += quantity
    }

    if colorCounts["blue"] > maxFoundBlue {
      maxFoundBlue = colorCounts["blue"]
    }
    if colorCounts["green"] > maxFoundGreen {
      maxFoundGreen = colorCounts["green"]
    }
    if colorCounts["red"] > maxFoundRed {
      maxFoundRed = colorCounts["red"]
    }
  }
  fmt.Printf(" id:%d \x1b[34mb:%d\x1b[0m \x1b[32mg:%d\x1b[0m \x1b[31mr:%d\x1b[0m\n", gameID, maxFoundBlue, maxFoundGreen, maxFoundRed)
  return gameID, maxFoundBlue, maxFoundGreen, maxFoundRed
}

func part2(game_input string) (int, int, int, int) {
  var minBlueRequired = 0
  var minGreenRequired = 0
  var minRedRequired = 0

  re := regexp.MustCompile(`(\d+) (\w+)`)

  input := strings.Split(game_input, ":")[1]
  hands := strings.Split(input, ";")

  for _, hand := range hands {
    colorCounts := make(map[string]int)
    matches := re.FindAllStringSubmatch(hand, -1)

    // Sum the quantities for each color
		for _, match := range matches {
			quantity, err := strconv.Atoi(match[1])
			if err != nil {
				fmt.Println("Error converting quantity to integer:", err)
				continue
			}

			color := match[2]
			colorCounts[color] += quantity
    }

    if colorCounts["blue"] > minBlueRequired {
      minBlueRequired = colorCounts["blue"]
    }
    if colorCounts["green"] > minGreenRequired {
      minGreenRequired = colorCounts["green"]
    }
    if colorCounts["red"] > minRedRequired {
      minRedRequired = colorCounts["red"]
    }
  }
  returnValue := minBlueRequired * minGreenRequired * minRedRequired
  return returnValue, minBlueRequired, minGreenRequired, minRedRequired
}

func main() {
  fmt.Println("Hello, World")
  var max_blue int = 14
  var max_green int = 13
  var max_red int = 12
  var sum int = 0

  file, err := os.Open("input.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
  defer file.Close()

  arg := os.Args[1]
  var operation func(string) (int, int, int, int)
  if arg == "1" {
    operation = part1
  } else {
    operation = part2
  }

  scanner := bufio.NewScanner(file)
  for scanner.Scan() {
		line := scanner.Text()
		fmt.Println(line)
    toSum, blue, green, red := operation(line)
    fmt.Printf(" id:%d \x1b[34mb:%d\x1b[0m \x1b[32mg:%d\x1b[0m \x1b[31mr:%d\x1b[0m\n", toSum, blue, green, red)

    if arg == "1" {
      if blue <= max_blue && green <= max_green && red <= max_red {
        sum += toSum
        fmt.Println(" Game: OK")
      } else {
        fmt.Println(" Game: NOT OK")
      }
    } else {
      sum += toSum
    }
	}
  
  if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}

  fmt.Println("Sum of game IDs:", sum)
}
