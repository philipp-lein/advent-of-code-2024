from collections import deque

surrounding = [(0, 1), (1, 0), (0, -1), (-1, 0)]


def main():
    print("Day 12")
    plant_map = read_input("input/input1.txt")
    plant_groups = get_plant_groups(plant_map)

    total_price = calculate_price(plant_groups)
    print(f"Total price of all plant groups: {total_price}")


def calculate_price(plant_groups):
    total_price = 0
    for group in plant_groups:
        total_price += calculate_perimeter(group) * len(group)
    return total_price


def calculate_perimeter(plant_group):
    """Calculate the perimeter of a plant group based on uncovered sides."""
    perimeter = 0
    for x, y in plant_group:
        for dx, dy in surrounding:
            neighbor = (x + dx, y + dy)
            if neighbor not in plant_group:
                perimeter += 1
    return perimeter


def read_input(file_path):
    """Read the input file and return the plant map."""
    try:
        with open(file_path, "r") as file:
            lines = [line.strip() for line in file.readlines()]
            if not lines:
                raise ValueError("The input file is empty.")
            return lines
    except FileNotFoundError:
        raise FileNotFoundError(f"The file {file_path} was not found.")


def get_plant_groups(plant_map):
    """Find all plant groups in the plant map."""
    plant_groups = []
    visited = set()
    for y in range(len(plant_map)):
        for x in range(len(plant_map[y])):
            if (x, y) not in visited:
                group = get_docking_fields(x, y, plant_map, visited)
                if group:
                    plant_groups.append(group)
    return plant_groups


def get_docking_fields(x, y, plant_map, visited) -> set[tuple[int, int]]:
    """Get all connected fields of the same plant type starting from (x, y)."""
    plant_type = plant_map[y][x]
    group = set()
    queue = deque([(x, y)])
    while queue:
        x, y = queue.popleft()
        if (x, y) in visited:
            continue
        visited.add((x, y))
        group.add((x, y))
        for dx, dy in surrounding:
            nx, ny = x + dx, y + dy
            # Check if the neighbor is within bounds and has the same plant type
            if 0 <= nx < len(plant_map[0]) and 0 <= ny < len(plant_map):
                if plant_map[ny][nx] == plant_type and (nx, ny) not in visited:
                    queue.append((nx, ny))
    return group


if __name__ == "__main__":
    main()
