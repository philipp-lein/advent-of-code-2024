fn main() {
    task1();
}
fn task1() {
    let mut lists = read_input("input/input1.txt");
    sort_lists(&mut lists);
    let distances = calculate_distances(lists);
    let sum: i32 = distances.iter().sum();
    println!("The sum of the distances is: {}", sum);
}
// read input from file parameter is the file path
fn read_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut result_list: Vec<Vec<i32>> = Vec::new();
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    let input = std::fs::read_to_string(file_path).unwrap();
    // split line 25228   50009 into two numbers
    input.lines().for_each(|line| {
        // split line into two numbers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        // add the first number to the firstList
        first_list.push(numbers[0]);
        // add the second number to the secondList
        second_list.push(numbers[1]);
    });

    result_list.push(first_list);
    result_list.push(second_list);

    return result_list;
}

// sort the instance of the lists
fn sort_lists(lists: &mut Vec<Vec<i32>>) {
    lists.iter_mut().for_each(|list| list.sort());
}

fn calculate_distances(lists: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result_list: Vec<i32> = Vec::new();
    let first_list = &lists[0];
    let second_list = &lists[1];
    for i in 0..first_list.len() {
        let distance = (first_list[i] - second_list[i]).abs();
        result_list.push(distance);
    }
    return result_list;
}
