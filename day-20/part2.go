package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

type Cell struct {
	Row, Col   int    // Row and column of the cell in the grid
	Symbol     rune   // The character representing the cell ('#', '.', 'S', 'E')
	Distance   int    // Distance from the starting cell, -1 if unvisited
}

var (
	grid           [][]Cell // The 2D grid of cells representing the input
	width, height  int      // Dimensions of the grid
	homeCell       *Cell    // Pointer to the starting cell ('S')
	goalCell       *Cell    // Pointer to the goal cell ('E')
	minimumEconomy = 100    // Minimum savings (in steps) required for a valid shortcut
	bestShortcuts  = 0      // Counter for the best shortcuts that meet the minimum economy
)

func main() {
	processInput()      // Parse the input file and initialize the grid
	walk()              // Compute the shortest distances from the starting cell
	findAllShortcuts()  // Identify all valid shortcuts
	fmt.Println("The answer is", bestShortcuts) // Output the result
}

// Reads the input grid from a file and initializes all cells
func processInput() {
	file, err := os.Open("input/input1.txt")
	if err != nil {
		panic(err) // Stop execution if the file cannot be opened
	}
	defer file.Close() // Ensure the file is closed after reading

	scanner := bufio.NewScanner(file)
	row := 0

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text()) // Remove unnecessary whitespace
		gridLine := []Cell{}
		for col, symbol := range line {
			cell := Cell{Row: row, Col: col, Symbol: symbol, Distance: -1}
			if symbol == 'S' {
				homeCell = &cell // Save a reference to the starting cell
			}
			if symbol == 'E' {
				goalCell = &cell // Save a reference to the goal cell
			}
			gridLine = append(gridLine, cell)
		}
		grid = append(grid, gridLine)
		row++
	}

	height = len(grid)
	if height > 0 {
		width = len(grid[0]) // Determine the grid width
	}
}

// Performs a breadth-first search (BFS) to calculate the shortest distances
// from the starting cell to all reachable cells in the grid
func walk() {
	if homeCell == nil {
		panic("Home cell not found") // Ensure there is a starting cell
	}

	homeCell.Distance = 0
	cellsToWalk := []*Cell{homeCell} // Initialize the BFS queue with the starting cell

	for len(cellsToWalk) > 0 {
		newCellsToWalk := []*Cell{} // Queue for the next BFS level
		for _, cell := range cellsToWalk {
			row, col := cell.Row, cell.Col
			distance := cell.Distance + 1 // Increment the distance

			// If the goal cell is reached, we can exit early
			if cell == goalCell {
				return
			}

			// Explore all four possible neighbors (up, down, left, right)
			grabCell(row-1, col, distance, &newCellsToWalk) // Up
			grabCell(row+1, col, distance, &newCellsToWalk) // Down
			grabCell(row, col-1, distance, &newCellsToWalk) // Left
			grabCell(row, col+1, distance, &newCellsToWalk) // Right
		}
		cellsToWalk = newCellsToWalk // Move to the next BFS level
	}
}

// Attempts to visit a neighboring cell during BFS
func grabCell(row, col, distance int, newCellsToWalk *[]*Cell) {
	// Check if the cell is out of bounds
	if row < 0 || col < 0 || row >= height || col >= width {
		return
	}

	cell := &grid[row][col]

	// Skip walls ('#') and cells that have already been visited
	if cell.Symbol == '#' || cell.Distance != -1 {
		return
	}

	cell.Distance = distance // Set the shortest distance to this cell
	*newCellsToWalk = append(*newCellsToWalk, cell) // Add the cell to the BFS queue
}

// Iterates over all reachable cells to identify shortcuts
func findAllShortcuts() {
	for _, line := range grid {
		for _, cell := range line {
			if cell.Distance != -1 { // Only consider cells that are reachable
				findShortcut(&cell) // Check for shortcuts from this cell
			}
		}
	}
}

// Identifies all valid shortcuts from a given cell
func findShortcut(baseCell *Cell) {
	// Iterate over all possible destination cells within a 20-step Manhattan distance
	for deltaRow := -20; deltaRow <= 20; deltaRow++ {
		row := baseCell.Row + deltaRow
		if row < 0 || row >= height {
			continue // Skip out-of-bounds rows
		}

		for deltaCol := -20; deltaCol <= 20; deltaCol++ {
			col := baseCell.Col + deltaCol
			if col < 0 || col >= width {
				continue // Skip out-of-bounds columns
			}

			// Calculate the Manhattan distance between the base cell and the destination cell
			manhattan := int(math.Abs(float64(deltaRow)) + math.Abs(float64(deltaCol)))
			if manhattan == 0 || manhattan > 20 {
				continue // Ignore the base cell itself and cells too far away
			}

			cell := &grid[row][col]
			if cell.Distance == -1 {
				continue // Skip unreachable cells
			}

			// Calculate the economy of using a shortcut: how many steps are saved
			economy := cell.Distance - baseCell.Distance - manhattan
			if economy >= minimumEconomy {
				bestShortcuts++ // Count the shortcut if it meets the minimum economy
			}
		}
	}
}
