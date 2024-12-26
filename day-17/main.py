def run_program(program, initial_registers):
    # Initialize registers
    registers = initial_registers.copy()

    # Instruction pointer
    ip = 0

    # Output storage
    output = []

    while ip < len(program):
        opcode = program[ip]
        operand = program[ip + 1] if ip + 1 < len(program) else None

        if opcode == 0:  # adv
            denominator = (
                2**operand if operand <= 3 else 2 ** registers["ABC"[operand - 4]]
            )
            registers["A"] //= denominator

        elif opcode == 1:  # bxl
            registers["B"] ^= operand

        elif opcode == 2:  # bst
            value = operand if operand <= 3 else registers["ABC"[operand - 4]]
            registers["B"] = value % 8

        elif opcode == 3:  # jnz
            if registers["A"] != 0:
                ip = operand
                continue  # Skip automatic increment

        elif opcode == 4:  # bxc
            registers["B"] ^= registers["C"]

        elif opcode == 5:  # out
            value = operand if operand <= 3 else registers["ABC"[operand - 4]]
            output.append(value % 8)

        elif opcode == 6:  # bdv
            denominator = (
                2**operand if operand <= 3 else 2 ** registers["ABC"[operand - 4]]
            )
            registers["B"] = registers["A"] // denominator

        elif opcode == 7:  # cdv
            denominator = (
                2**operand if operand <= 3 else 2 ** registers["ABC"[operand - 4]]
            )
            registers["C"] = registers["A"] // denominator

        # Move to next instruction
        ip += 2

    return ",".join(map(str, output))


# Example input
program = [0, 1, 5, 4, 3, 0]  # Replace with your puzzle input
initial_registers = {"A": 729, "B": 0, "C": 0}  # Replace with provided register values

# Run the program
result = run_program(program, initial_registers)
print(result)
