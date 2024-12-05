fn main() {
    stage1();
    stage2();
}

fn stage1() {
    let (page_ordering_rules, pages) = read_input("input/input1.txt");

    let pages_in_right_order = get_pages_which_are_in_right_order(page_ordering_rules, pages);
    println!("Pages in right order: {:?}", pages_in_right_order);

    let middle_pages = get_middle_pages(pages_in_right_order);
    println!("Middle pages: {:?}", middle_pages);

    let sum_of_middle_pages: i32 = middle_pages.iter().sum();
    println!("Sum of middle pages: {:?} (Stage1)", sum_of_middle_pages);
}

fn stage2() {
    let (page_ordering_rules, pages) = read_input("input/input1.txt");

    let pages_not_in_right_order =
        get_pages_which_are_not_in_right_order(&page_ordering_rules, &pages);
    println!("Pages not in right order: {:?}", pages_not_in_right_order);

    let pages_in_right_order =
        bring_pages_in_right_order(page_ordering_rules, pages_not_in_right_order);
    println!("Pages are now in right order: {:?}", pages_in_right_order);

    let middle_pages: Vec<i32> = get_middle_pages(pages_in_right_order);
    println!("Middle pages: {:?}", middle_pages);

    let sum_of_middle_pages: i32 = middle_pages.iter().sum();
    println!("Sum of middle pages: {:?} (Stage2)", sum_of_middle_pages);
}

fn read_input(file_path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut page_ordering_rules: Vec<(i32, i32)> = Vec::new();
    let mut pages: Vec<Vec<i32>> = Vec::new();
    let input = std::fs::read_to_string(file_path).unwrap();
    let mut is_page_ordering_rules = true;
    input.lines().for_each(|line| {
        if line.is_empty() {
            is_page_ordering_rules = false;
        } else if is_page_ordering_rules {
            let numbers: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();
            page_ordering_rules.push((numbers[0], numbers[1]));
        } else {
            let page_lines: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            pages.push(page_lines);
        }
    });
    return (page_ordering_rules, pages);
}

fn get_pages_which_are_in_right_order(
    page_ordering_rules: Vec<(i32, i32)>,
    page_lines: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut pages_in_right_order: Vec<Vec<i32>> = Vec::new();

    for page_line in page_lines {
        let mut is_in_right_order = true;

        for (page_which_should_be_before, page_which_should_be_after) in &page_ordering_rules {
            // check if both numbers are in the page_line
            let is_page_which_should_be_before_in_page_line =
                page_line.contains(page_which_should_be_before);
            let is_page_which_should_be_after_in_page_line =
                page_line.contains(page_which_should_be_after);

            // if both numbers are in the page_line, then check if they are in the right order
            if is_page_which_should_be_before_in_page_line
                && is_page_which_should_be_after_in_page_line
            {
                let index_of_page_which_should_be_before = page_line
                    .iter()
                    .position(|&x| x == *page_which_should_be_before)
                    .unwrap();
                let index_of_page_which_should_be_after = page_line
                    .iter()
                    .position(|&x| x == *page_which_should_be_after)
                    .unwrap();
                if index_of_page_which_should_be_before > index_of_page_which_should_be_after {
                    is_in_right_order = false;
                    break;
                }
            }
        }

        // Add the page_line if it satisfies all rules
        if is_in_right_order {
            pages_in_right_order.push(page_line.clone());
        }
    }

    return pages_in_right_order;
}

fn get_pages_which_are_not_in_right_order(
    page_ordering_rules: &Vec<(i32, i32)>,
    page_lines: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut pages_not_in_right_order: Vec<Vec<i32>> = Vec::new();

    for page_line in page_lines {
        let is_in_right_order =
            check_if_page_line_is_in_right_order(page_line, page_ordering_rules);
        if !is_in_right_order {
            pages_not_in_right_order.push(page_line.clone());
        }
    }

    return pages_not_in_right_order;
}

fn check_if_page_line_is_in_right_order(
    page_line: &Vec<i32>,
    page_ordering_rules: &Vec<(i32, i32)>,
) -> bool {
    let mut is_in_right_order = true;

    for (page_which_should_be_before, page_which_should_be_after) in page_ordering_rules {
        // check if both numbers are in the page_line
        let is_page_which_should_be_before_in_page_line =
            page_line.contains(page_which_should_be_before);
        let is_page_which_should_be_after_in_page_line =
            page_line.contains(page_which_should_be_after);

        // if both numbers are in the page_line, then check if they are in the right order
        if is_page_which_should_be_before_in_page_line && is_page_which_should_be_after_in_page_line
        {
            let index_of_page_which_should_be_before = page_line
                .iter()
                .position(|&x| x == *page_which_should_be_before)
                .unwrap();
            let index_of_page_which_should_be_after = page_line
                .iter()
                .position(|&x| x == *page_which_should_be_after)
                .unwrap();
            if index_of_page_which_should_be_before > index_of_page_which_should_be_after {
                is_in_right_order = false;
                break;
            }
        }
    }

    return is_in_right_order;
}

fn get_middle_pages(pages_in_right_order: Vec<Vec<i32>>) -> Vec<i32> {
    let mut middle_pages: Vec<i32> = Vec::new();
    for page_line in pages_in_right_order {
        let middle_index = page_line.len() / 2;
        middle_pages.push(page_line[middle_index]);
    }
    return middle_pages;
}

fn bring_pages_in_right_order(
    page_ordering_rules: Vec<(i32, i32)>,
    pages_not_in_right_order: Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
    let mut pages_in_right_order: Vec<Vec<i32>> = Vec::new();

    // Iterate over page lines and reorder them
    for page_line in pages_not_in_right_order {
        let mut new_page_line = page_line.clone();

        while !check_if_page_line_is_in_right_order(&new_page_line, &page_ordering_rules) {
            for (page_which_should_be_before, page_which_should_be_after) in &page_ordering_rules {
                if let (Some(index_of_before), Some(index_of_after)) = (
                    new_page_line
                        .iter()
                        .position(|&x| x == *page_which_should_be_before),
                    new_page_line
                        .iter()
                        .position(|&x| x == *page_which_should_be_after),
                ) {
                    if index_of_before > index_of_after {
                        new_page_line.swap(index_of_before, index_of_after);
                    }
                }
            }
        }
        pages_in_right_order.push(new_page_line);
    }

    return pages_in_right_order;
}
