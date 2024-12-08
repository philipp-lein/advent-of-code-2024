def main(input_file)
    # Read and parse the input file
    formulas = parse_input(input_file)
  
    # Find the sum of test values of valid equations
    total_calibration_result = formulas.sum do |test_value, numbers|
      valid?(test_value, numbers) ? test_value : 0
    end
  
    puts "Total calibration result: #{total_calibration_result}"
  end
  
  def parse_input(file_path)
    # Parse the input into a list of [test_value, numbers] pairs
    File.readlines(file_path).map do |line|
      test_value, numbers = line.split(":").map(&:strip)
      [test_value.to_i, numbers.split.map(&:to_i)]
    end
  end
  
  def valid?(test_value, numbers)
    # Generate all possible combinations of operators
    operators = ["+", "*"]
    operator_combinations = operators.repeated_permutation(numbers.size - 1)
  
    # Check if any combination of operators makes the equation true
    operator_combinations.any? do |ops|
      result = evaluate(numbers, ops)
      result == test_value
    end
  end
  
  def evaluate(numbers, operators)
    # Evaluate the equation with the given operators, left to right
    result = numbers.first
    operators.each_with_index do |op, i|
      case op
      when "+"
        result += numbers[i + 1]
      when "*"
        result *= numbers[i + 1]
      end
    end
    result
  end
  
  # Call the main function with the input file
  main("input/input1.txt")
  