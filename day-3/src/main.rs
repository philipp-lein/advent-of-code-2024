use regex::Regex;

fn main() {
    stage1();
    stage2();
}

fn stage1() {
    let instructions_string = read_input("input/input1.txt");
    let valid_instructions = get_valid_instructions(&instructions_string);
    let multiplication_product = run_instructions(&valid_instructions);
    println!(
        "The multiplication product is: {:?}",
        multiplication_product
    );
}

fn stage2() {
    let instructions_string = read_input("input/input1.txt");
    let valid_instructions = get_valid_instructions_with_do_and_dont(&instructions_string);
    let multiplication_product = run_instructions(&valid_instructions);
    println!(
        "The multiplication product is: {:?}",
        multiplication_product
    );
}

fn read_input(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap()
}

fn get_valid_instructions(instructions_string: &str) -> Vec<Vec<i64>> {
    // Find per regex mul(number,number)
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut instructions: Vec<Vec<i64>> = Vec::new();
    for cap in re.captures_iter(instructions_string) {
        let first_number = cap[1].parse::<i64>().unwrap();
        let second_number = cap[2].parse::<i64>().unwrap();
        instructions.push(vec![first_number, second_number]);
    }
    instructions
}

fn run_instructions(instructions: &Vec<Vec<i64>>) -> i64 {
    let mut result: i64 = 0;
    for instruction in instructions {
        result += instruction[0] * instruction[1];
    }
    result
}

fn get_valid_instructions_with_do_and_dont(instructions_string: &str) -> Vec<Vec<i64>> {
    // regex for mul(number,number) or do or dont
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't|do").unwrap();
    let mut instructions: Vec<Vec<i64>> = Vec::new();
    let mut should_be_done = true;
    for capture in re.captures_iter(instructions_string) {
        if capture.get(0).unwrap().as_str() == "do" {
            should_be_done = true;
        } else if capture.get(0).unwrap().as_str() == "don't" {
            should_be_done = false;
        } else {
            let first_number = capture[1].parse::<i64>().unwrap();
            let second_number = capture[2].parse::<i64>().unwrap();
            if should_be_done {
                instructions.push(vec![first_number, second_number]);
            }
        }
    }
    return instructions;
}
