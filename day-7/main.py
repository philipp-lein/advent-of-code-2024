from itertools import product

def main(input_file):
    # Parse the input file
    formulas = parse_input(input_file)

    # Find the sum of test values of valid equations
    total_calibration_result = sum(
        test_value if is_valid(test_value, numbers) else 0
        for test_value, numbers in formulas
    )

    print(f"Total calibration result: {total_calibration_result}")


def parse_input(file_path):
    # Parse the input into a list of (test_value, numbers) pairs
    with open(file_path, "r") as f:
        lines = f.readlines()

    formulas = []
    for line in lines:
        test_value, numbers = line.split(":")
        test_value = int(test_value.strip())
        numbers = list(map(int, numbers.strip().split()))
        formulas.append((test_value, numbers))
    
    return formulas


def is_valid(test_value, numbers):
    # Generate all possible combinations of operators
    operators = ["+", "*"]
    operator_combinations = product(operators, repeat=len(numbers) - 1)

    # Check if any combination of operators makes the equation true
    for ops in operator_combinations:
        if evaluate(numbers, ops) == test_value:
            return True
    return False


def evaluate(numbers, operators):
    # Evaluate the equation with the given operators, left to right
    result = numbers[0]
    for i, op in enumerate(operators):
        if op == "+":
            result += numbers[i + 1]
        elif op == "*":
            result *= numbers[i + 1]
    return result


# Call the main function with the input file
if __name__ == "__main__":
    main("input/input1.txt")
