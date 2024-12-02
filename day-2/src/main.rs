fn main() {
    stage1();
    stage2();
}
fn stage1() {
    let reports = read_input("input/input1.txt");
    let safe_reports_count = get_safe_reports(reports).len();
    println!("There are {:?} valid reports.", safe_reports_count);
}

fn stage2() {
    let reports = read_input("input/input1.txt");
    let safe_reports_count = get_safe_reports_when_one_level_could_be_replaced(reports).len();
    println!(
        "There are {:?} valid reports (when you can at least remove one level).",
        safe_reports_count
    );
}

// read input from file parameter is the file path
fn read_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut result_list: Vec<Vec<i32>> = Vec::new();
    let input = std::fs::read_to_string(file_path).unwrap();
    input.lines().for_each(|line| {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        // add the first number to the firstList
        result_list.push(levels);
    });

    return result_list;
}

fn get_safe_reports(reports: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut safe_reports: Vec<Vec<i32>> = Vec::new();
    for report in reports {
        if check_if_report_is_safe(&report) {
            safe_reports.push(report);
        }
    }
    return safe_reports;
}

fn get_safe_reports_when_one_level_could_be_replaced(reports: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // If you can replace one level, then the report is safe.

    let mut safe_reports: Vec<Vec<i32>> = Vec::new();
    for report in reports {
        if check_if_report_is_safe(&report) {
            safe_reports.push(report);
        } else {
            // Second chance :D try all other possibilities
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if check_if_report_is_safe(&new_report) {
                    safe_reports.push(new_report);
                    break;
                }
            }
        }
    }
    return safe_reports;
}

fn check_if_report_is_safe(report: &Vec<i32>) -> bool {
    // safe if
    // The levels are either all increasing or all decreasing.
    // Any two adjacent levels differ by at least one and at most three.
    // eg 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.

    let mut is_safe = true;
    let is_ascending = report[0] < report[1];
    for i in 0..report.len() - 1 {
        if (report[i] < report[i + 1]) != is_ascending {
            is_safe = false;
            break;
        }
        let diff = (report[i] - report[i + 1]).abs();
        if diff < 1 || diff > 3 {
            is_safe = false;
            break;
        }
    }
    return is_safe;
}
