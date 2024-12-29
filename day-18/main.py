from collections import deque

GRID_SIZE = 71
HOW_MANY_BYTES = 1024


def main():
    print("Hello, Advent of Code 2024 - Day 18!")

    falling_bytes = get_falling_bytes("input/input1.txt")
    falling_bytes = falling_bytes[:HOW_MANY_BYTES]
    memory_grid = get_memory_grid(falling_bytes)
    print("\n".join("".join(row) for row in memory_grid))

    steps = find_shortest_path(memory_grid)
    if steps == -1:
        print("No path found!")
    else:
        print(f"You need {steps} steps")


def get_memory_grid(falling_bytes: list[tuple[int, int]]) -> list[list[str]]:
    grid = [["." for _ in range(GRID_SIZE)] for _ in range(GRID_SIZE)]
    for x, y in falling_bytes:
        if y >= GRID_SIZE or x >= GRID_SIZE:
            continue
        grid[y][x] = "#"
    return grid


def get_falling_bytes(file_path):
    result: list[tuple[int, int]] = []
    with open(file_path, "r") as file:
        for line in file:
            result.append(tuple(map(int, line.strip().split(","))))
    return result


def find_shortest_path(memory_grid: list[list[str]]) -> int:
    start = (0, 0)
    queue = deque([(start, 0)])  # (position, steps)
    visited: dict[tuple[int, int], int] = {}

    while queue:
        (x, y), steps = queue.popleft()

        # Skip if out of bounds or corrupted
        if memory_grid[y][x] == "#":
            continue

        # Goal condition
        if (x, y) == (GRID_SIZE - 1, GRID_SIZE - 1):
            return steps

        # Skip if visited with fewer or equal steps
        if (x, y) in visited and visited[(x, y)] <= steps:
            continue

        # Mark as visited
        visited[(x, y)] = steps

        # Add neighbors with boundary checks
        if y < GRID_SIZE - 1:
            queue.append(((x, y + 1), steps + 1))  # Down
        if y > 0:
            queue.append(((x, y - 1), steps + 1))  # Up
        if x > 0:
            queue.append(((x - 1, y), steps + 1))  # Left
        if x < GRID_SIZE - 1:
            queue.append(((x + 1, y), steps + 1))  # Right

    # No path found
    return -1


if __name__ == "__main__":
    main()
