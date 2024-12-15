from collections import deque


def main():
    print("Day 13")
    machines = read_input("input/input1.txt")
    print(f"Machines: {machines}")
    prizes_won, total_cost = calculate_min_tokens(machines)
    print(f"Prizes won: {prizes_won}")
    print(f"Total cost: {total_cost}")


def read_input(filename):
    machines = []
    with open(filename, "r") as file:
        while True:
            try:
                # Read Button A line
                line = file.readline().strip()
                if not line:  # End of file
                    break
                _, a_details = line.split(": ")
                ax, ay = map(int, [v.split("+")[1] for v in a_details.split(", ")])

                # Read Button B line
                line = file.readline().strip()
                _, b_details = line.split(": ")
                bx, by = map(int, [v.split("+")[1] for v in b_details.split(", ")])

                # Read Prize line
                line = file.readline().strip()
                _, p_details = line.split(": ")
                px, py = map(int, [v.split("=")[1] for v in p_details.split(", ")])

                # Store machine details
                machines.append({"A": (ax, ay), "B": (bx, by), "Prize": (px, py)})

                # Skip the empty line between blocks
                file.readline()

            except Exception as e:
                print(f"Error parsing file: {e}")
                break
    return machines


def calculate_min_tokens(machines):
    total_cost = 0
    prizes_won = 0

    for machine in machines:
        a_move = machine["A"]
        b_move = machine["B"]
        prize = machine["Prize"]

        # Use BFS to find the minimum cost
        queue = deque([(0, 0)])  # a, b counters
        visited = set()
        min_cost = float("inf")
        best_solution = None

        while queue:
            a, b = queue.popleft()
            if (a, b) in visited:
                continue
            visited.add((a, b))

            # Calculate positions
            x = a * a_move[0] + b * b_move[0]
            y = a * a_move[1] + b * b_move[1]

            # Check if we've hit the target
            if x == prize[0] and y == prize[1]:
                cost = 3 * a + b
                if cost < min_cost:
                    min_cost = cost
                    best_solution = (a, b)

            # If not, continue exploring
            if x <= prize[0] and y <= prize[1]:  # Stay within bounds
                queue.append((a + 1, b))  # Add one A press
                queue.append((a, b + 1))  # Add one B press

        if best_solution:
            prizes_won += 1
            total_cost += min_cost
            print(
                f"Machine: {machine}, Solution: {best_solution}, Cost: {min_cost} tokens"
            )
        else:
            print(f"No solution found for machine: {machine}")

    return prizes_won, total_cost


if __name__ == "__main__":
    main()
