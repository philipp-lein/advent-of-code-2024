from collections import deque

surrounding = [(0, 1), (1, 0), (0, -1), (-1, 0)]

def main():
    print("Day 10")
    hike_map = read_input("input/input1.txt")
    trailheads = find_trailheads(hike_map)
    total_score = sum([score_trailhead(x, y, hike_map) for x, y in trailheads])
    print(f"Total score of all trailheads: {total_score}")

def find_trailheads(hike_map):
    """Find all trailheads (positions with height 0)."""
    trailheads = []
    for y in range(len(hike_map)):
        for x in range(len(hike_map[y])):
            if hike_map[y][x] == 0:
                trailheads.append((x, y))
    return trailheads

def score_trailhead(x, y, hike_map):
    """Calculate the score of a trailhead."""
    visited = set()
    queue = deque([(x, y)])
    score = 0

    while queue:
        cx, cy = queue.popleft()
        if (cx, cy) in visited:
            continue
        visited.add((cx, cy))

        current_height = hike_map[cy][cx]
        if current_height == 9:
            score += 1  # Reached a valid `9` position

        for dx, dy in surrounding:
            nx, ny = cx + dx, cy + dy
            if 0 <= nx < len(hike_map[0]) and 0 <= ny < len(hike_map):
                neighbor_height = hike_map[ny][nx]
                if neighbor_height == current_height + 1 and (nx, ny) not in visited:
                    queue.append((nx, ny))

    return score

def read_input(file_path):
    """Read the input file and return the hike map."""
    with open(file_path, "r") as file:
        return [list(map(int, line.strip())) for line in file.readlines()]

if __name__ == "__main__":
    main()
