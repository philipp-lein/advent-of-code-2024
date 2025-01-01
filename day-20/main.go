package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

func main() {
	filePath := "input/input1.txt"
	raceMap, start, end, error := readFile(filePath)
	if error != nil {
		fmt.Println(error)
		return
	}
	picoSecondsNormalRun := findTheWay(raceMap, start, end)

	cheatList := getAllSolutionsWithCheatingOneTime(raceMap, start, end, picoSecondsNormalRun)
	prettyPrint(cheatList, picoSecondsNormalRun)
}

func prettyPrint(cheatList []int, picoSecondsNormalRun int) {
	// Calculate the time saved by each cheat
	timeSaves := []int{}
	for _, cheatTime := range cheatList {
		timeSaved := picoSecondsNormalRun - cheatTime
		timeSaves = append(timeSaves, timeSaved)
	}

	// Group the cheats by time saved
	timeSaveCounts := make(map[int]int)
	for _, save := range timeSaves {
		timeSaveCounts[save]++
	}

	// Sort the keys of the map for ordered output
	sortedTimeSaves := make([]int, 0, len(timeSaveCounts))
	for save := range timeSaveCounts {
		sortedTimeSaves = append(sortedTimeSaves, save)
	}
	sort.Ints(sortedTimeSaves)

	// Print grouped cheats summary
	fmt.Println("Each cheat has a distinct start position (the position where the cheat is activated, just before the first move that is allowed to go through walls) and end position; cheats are uniquely identified by their start position and end position.\n")
	fmt.Println("In this example, the total number of cheats (grouped by the amount of time they save) are as follows:\n")

	for _, save := range sortedTimeSaves {
		count := timeSaveCounts[save]
		if count == 1 {
			fmt.Printf("There is one cheat that saves %d picoseconds.\n", save)
		} else {
			fmt.Printf("There are %d cheats that save %d picoseconds.\n", count, save)
		}
	}

	// Calculate the number of cheats that save at least 100 picoseconds
	cheatsSavingAtLeast100 := 0
	for _, save := range timeSaves {
		if save >= 100 {
			cheatsSavingAtLeast100++
		}
	}

	fmt.Printf("\nYou aren't sure what the conditions of the racetrack will be like, so to give yourself as many options as possible, you'll need a list of the best cheats. How many cheats would save you at least 100 picoseconds?\n")
	fmt.Printf("Answer: %d\n", cheatsSavingAtLeast100)
}

func getAllSolutionsWithCheatingOneTime(raceMap [][]string, start, end string, normalTime int) []int {
	results := []int{}
	startRow, startCol := parseCoordinates(start)
	endRow, endCol := parseCoordinates(end)

	directions := []struct {
		dr, dc int
	}{
		{-1, 0}, // up
		{1, 0},  // down
		{0, -1}, // left
		{0, 1},  // right
	}

	type State struct {
		row, col, time             int
		cheatFromRow, cheatFromCol int // Position from which the cheat was executed, or -1, -1 if no cheat used
	}

	queue := []State{{startRow, startCol, 0, -1, -1}}
	visited := make(map[string]bool)
	visited[fmt.Sprintf("%d,%d,%d,%d", startRow, startCol, -1, -1)] = true

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if current.cheatFromRow != -1 && current.cheatFromCol != -1 && current.time+100 > normalTime {
			continue
		}

		if current.row == endRow && current.col == endCol {
			results = append(results, current.time)
			continue
		}

		for _, d := range directions {
			newRow, newCol := current.row+d.dr, current.col+d.dc

			if newRow >= 0 && newRow < len(raceMap) &&
				newCol >= 0 && newCol < len(raceMap[0]) {
				// normal run
				key := fmt.Sprintf("%d,%d,%d,%d", newRow, newCol, current.cheatFromRow, current.cheatFromCol)
				if raceMap[newRow][newCol] != "#" && !visited[key] {
					visited[key] = true
					queue = append(queue, State{newRow, newCol, current.time + 1, current.cheatFromRow, current.cheatFromCol})
				}

				// cheat logic
				if current.cheatFromRow == -1 && raceMap[newRow][newCol] == "#" {
					cheatRow, cheatCol := newRow+d.dr, newCol+d.dc
					if cheatRow >= 0 && cheatRow < len(raceMap) &&
						cheatCol >= 0 && cheatCol < len(raceMap[0]) &&
						(raceMap[cheatRow][cheatCol] == "." || raceMap[cheatRow][cheatCol] == "E") {

						cheatKey := fmt.Sprintf("%d,%d,%d,%d", cheatRow, cheatCol, newRow, newCol)
						if !visited[cheatKey] {
							visited[cheatKey] = true
							queue = append(queue, State{cheatRow, cheatCol, current.time + 2, newRow, newCol})
						}
					}
				}
			}
		}
	}
	return results
}

func findTheWay(raceMap [][]string, start, end string) int {
	startRow, startCol := parseCoordinates(start)
	endRow, endCol := parseCoordinates(end)

	directions := []struct {
		dr, dc int
	}{
		{-1, 0}, // up
		{1, 0},  // down
		{0, -1}, // left
		{0, 1},  // right
	}

	queue := []struct {
		row, col, time int
	}{{startRow, startCol, 0}}

	visited := make(map[string]bool)
	visited[fmt.Sprintf("%d,%d", startRow, startCol)] = true

	for len(queue) > 0 {
		current := queue[0]
		queue = queue[1:]

		if current.row == endRow && current.col == endCol {
			return current.time
		}

		for _, d := range directions {
			newRow, newCol := current.row+d.dr, current.col+d.dc

			if newRow >= 0 && newRow < len(raceMap) &&
				newCol >= 0 && newCol < len(raceMap[0]) &&
				raceMap[newRow][newCol] != "#" {
				newPos := fmt.Sprintf("%d,%d", newRow, newCol)
				if !visited[newPos] {
					visited[newPos] = true
					queue = append(queue, struct {
						row, col, time int
					}{newRow, newCol, current.time + 1})
				}
			}
		}
	}

	return -1 // Return -1 if no path is found
}

func parseCoordinates(coords string) (row, col int) {
	fmt.Sscanf(coords, "%d,%d", &row, &col)
	return
}

func readFile(filePath string) (raceMap [][]string, start, end string, err error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, "", "", fmt.Errorf("failed to open file: %w", err)
	}
	defer file.Close()

	var startCoords, endCoords string
	var lines []string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, "", "", fmt.Errorf("error reading file: %w", err)
	}

	raceMap = make([][]string, len(lines))
	for i, line := range lines {
		raceMap[i] = strings.Split(line, "")
		for j, char := range raceMap[i] {
			if char == "S" {
				startCoords = fmt.Sprintf("%d,%d", i, j)
			} else if char == "E" {
				endCoords = fmt.Sprintf("%d,%d", i, j)
			}
		}
	}

	if startCoords == "" {
		return nil, "", "", fmt.Errorf("start position 'S' not found")
	}
	if endCoords == "" {
		return nil, "", "", fmt.Errorf("end position 'E' not found")
	}

	return raceMap, startCoords, endCoords, nil
}
