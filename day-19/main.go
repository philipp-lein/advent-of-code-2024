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
	towelPatterns, designs, err := readInput(filePath)
	if err != nil {
		fmt.Println("Error reading input file")
		return
	}

	possibleDesigns := getPossibleDesigns(towelPatterns, designs)
	fmt.Println("Possible designs:", possibleDesigns, "Count:", len(possibleDesigns))
}

func getPossibleDesigns(towelPatterns []string, designs []string) (possibleDesigns []string) {
	possibleDesigns = make([]string, 0)
	// sort towel patterns by length in descending order
	sort.Slice(towelPatterns, func(i, j int) bool {
		return len(towelPatterns[i]) > len(towelPatterns[j])
	})

	cache := make(map[string]int)

	for _, design := range designs {
		if isPossibleDesign(design, towelPatterns, cache) {
			possibleDesigns = append(possibleDesigns, design)
		}
	}
	return possibleDesigns
}

func isPossibleDesign(design string, towelPatterns []string, cache map[string]int) bool {
	if value, exists := cache[design]; exists {
		println("Cache hit for", design, "value:", value)
		return value > 0
	}

	matchString := design
	count := 0
	for _, pattern := range towelPatterns {
		if strings.HasPrefix(matchString, pattern) {
			remaining := strings.TrimPrefix(matchString, pattern)
			if len(remaining) == 0 || isPossibleDesign(remaining, towelPatterns, cache) {
				count++
			}
		}
	}

	cache[design] = count
	return count > 0
}

func readInput(filePath string) ([]string, []string, error) {
	// Open the file
	file, err := os.Open(filePath)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return nil, nil, err
	}
	defer file.Close()

	// Read the file content
	scanner := bufio.NewScanner(file)

	var patterns []string
	var designs []string

	lineNumber := 0
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())

		if lineNumber == 0 {
			// First line contains patterns
			patterns = strings.Split(line, ", ")
		} else {
			// Other lines contain designs
			// ignore empty lines
			if len(line) == 0 {
				continue
			}

			designs = append(designs, line)
		}
		lineNumber++
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return nil, nil, err
	}
	return patterns, designs, nil
}
