package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

const FreeSpace = -1

func main() {
	filePath := "input/input1.txt"
	input, err := readInput(filePath)
	if err != nil {
		fmt.Println("Error reading input:", err)
		return
	}

	checksum := solve(input)
	fmt.Println("Checksum:", checksum)
}

func readInput(filePath string) ([]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %v", err)
	}
	defer file.Close()

	var input []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading file: %v", err)
	}

	return input, nil
}

func cleanBlock(disk []int) []int {
	left := 0              // Pointer to the next free space from the left
	right := len(disk) - 1 // Pointer to the next file block from the right

	for left < right {
		// Move the left pointer to the next free space
		for left < len(disk) && disk[left] != FreeSpace {
			left++
		}

		// Move the right pointer to the next file block
		for right >= 0 && disk[right] == FreeSpace {
			right--
		}

		// If pointers cross, stop
		if left >= right {
			break
		}

		// Swap the free space and the file block
		disk[left], disk[right] = disk[right], disk[left]

		// Move pointers inward
		left++
		right--
	}

	return disk
}

func solve(input []string) int {
	disk := loadInput(input)

	// Clean the disk using cleanBlock
	disk = cleanBlock(disk)

	// Calculate checksum after cleaning
	return calculateChecksum(disk)
}

func calculateChecksum(disk []int) int {
	checksum := 0
	for i, block := range disk {
		if block != FreeSpace {
			checksum += i * block
		}
	}
	return checksum
}

func loadInput(input []string) []int {
	if len(input) == 0 {
		return nil
	}

	line := input[0]
	var disk []int
	id := -1
	readingFile := true

	for _, char := range line {
		count, err := strconv.Atoi(string(char))
		if err != nil {
			fmt.Println("Error parsing input:", err)
			return nil
		}

		if readingFile {
			id++
			for i := 0; i < count; i++ {
				disk = append(disk, id)
			}
		} else {
			for i := 0; i < count; i++ {
				disk = append(disk, FreeSpace)
			}
		}
		readingFile = !readingFile
	}

	return disk
}
