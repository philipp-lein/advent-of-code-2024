package main

import (
	"bufio"
	"fmt"
	"os"
)

var DIRECTIONS = []rune{'^', '>', 'v', '<'} // Clockwise order

// Movement deltas for each direction
var MOVES = map[rune][2]int{
	'^': {-1, 0}, // Up
	'v': {1, 0},  // Down
	'<': {0, -1}, // Left
	'>': {0, 1},  // Right
}

func main() {
	// Load the game map from the file
	gameMap, startPosition, err := readGameMap("input/input1.txt")
	if err != nil {
		fmt.Println("Error reading the map:", err)
		return
	}

	// Walk through the map and calculate distinct positions visited
	visitedPositions := walkMap(gameMap, startPosition)

	// Display the walked map with marked positions
	fmt.Println("Walked map:")
	for _, row := range gameMap {
		fmt.Println(string(row))
	}

	// Output the result
	fmt.Println("Distinct positions visited:", len(visitedPositions))
}

// Function to read the game map from a file
func readGameMap(filename string) ([][]rune, [2]int, error) {
	startPosition := [2]int{-1, -1}
	file, err := os.Open(filename)
	if err != nil {
		return nil, startPosition, err
	}
	defer file.Close()

	var gameMap [][]rune
	yPosition := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if startPosition[0] == -1 {
			startPosition = findStartPosition(line, yPosition)
		}
		row := []rune(line)
		gameMap = append(gameMap, row)
		yPosition++
	}
	if err := scanner.Err(); err != nil {
		return nil, startPosition, err
	}
	return gameMap, startPosition, nil
}

func findStartPosition(line string, y int) [2]int {
	for x, char := range line {
		if char == '^' || char == 'v' || char == '<' || char == '>' {
			return [2]int{x, y}
		}
	}
	return [2]int{-1, -1}
}

func walkMap(gameMap [][]rune, startPosition [2]int) map[[2]int]bool {
	x, y := startPosition[0], startPosition[1]
	direction := gameMap[y][x] // Initial direction

	visited := make(map[[2]int]bool)
	visited[[2]int{x, y}] = true
	gameMap[y][x] = 'X'

	for {
		// Attempt to move in the current direction
		move := MOVES[direction]
		newX, newY := x+move[1], y+move[0]

		// Check if the new position is out of bounds
		if newY < 0 || newY >= len(gameMap) || newX < 0 || newX >= len(gameMap[0]) {
			break
		}

		// Check the field at the new position
		nextField := gameMap[newY][newX]
		if nextField == '#' { // Obstacle, turn right
			direction = turnRight(direction)
		} else { // Move forward
			x, y = newX, newY
			if !visited[[2]int{x, y}] {
				visited[[2]int{x, y}] = true
				gameMap[y][x] = 'X'
			}
		}
	}

	return visited
}

func turnRight(currentDirection rune) rune {
	for i, dir := range DIRECTIONS {
		if dir == currentDirection {
			return DIRECTIONS[(i+1)%len(DIRECTIONS)]
		}
	}
	return currentDirection
}
