use std::fs;

fn main() {
    stage1();
}

fn stage1() {
    let list_of_characters = read_input("input/input1.txt");
    let count_the_word_xmas = count_the_word_xmas(list_of_characters);
    println!("There are {:?} XMAS words.", count_the_word_xmas);
}

fn stage2() {
    let list_of_characters = read_input("input/input1.txt");
    let count_the_word_cross_mass = count_the_word_cross_mas(list_of_characters);
    println!("There are {:?} Cross MAS words.", count_the_word_xmas);
}

fn read_input(file_path: &str) -> Vec<String> {
    let input = fs::read_to_string(file_path).unwrap();
    input.lines().map(|line| line.to_string()).collect()
}

fn count_the_word_xmas(list_of_characters: Vec<String>) -> i32 {
    let mut count = 0;

    // Define all search directions
    let directions = vec![
        search_to_the_right,
        search_to_the_left,
        search_to_the_bottom,
        search_to_the_top,
        search_to_the_bottom_right,
        search_to_the_bottom_left,
        search_to_the_top_right,
        search_to_the_top_left,
    ];

    for (y, word) in list_of_characters.iter().enumerate() {
        for (x, character) in word.chars().enumerate() {
            if character == 'X' {
                for search_fn in &directions {
                    if search_fn(x, y, &list_of_characters, "MAS") {
                        count += 1;
                    }
                }
            }
        }
    }

    return count
}

fn count_the_word_cross(list_of_characters: Vec<String>) -> i32 {
    let mut count = 0;

    // search for
    // M.S
    // .A.
    // M.S
    //
    //

    // Define all search directions
    let directions = vec![
        search_to_the_bottom_right,
        search_to_the_bottom_left,
        search_to_the_top_right,
        search_to_the_top_left,
    ];

    for (y, word) in list_of_characters.iter().enumerate() {
        for (x, character) in word.chars().enumerate() {
            if character == 'M' {
                for search_fn in &directions {
                    if search_fn(x, y, &list_of_characters, "AS") {
                        if search_fn == &search_to_the_bottom_right {
                            // search for other direction cross
                            if search_to_the_top_right(x, y+2, &list_of_characters, "AS") {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    return count
}

fn search_to_the_right(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        let line = &list_of_characters[y];
        if x + 1 < line.len() && line.chars().nth(x + 1) == Some(character_to_search) {
            let new_search_state: String = search_state.chars().skip(1).collect();
            return search_to_the_right(x + 1, y, list_of_characters, &new_search_state);
        }
    }
    return false
}

fn search_to_the_left(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        let line = &list_of_characters[y];
        if x > 0 && line.chars().nth(x - 1) == Some(character_to_search) {
            let new_search_state: String = search_state.chars().skip(1).collect();
            return search_to_the_left(x - 1, y, list_of_characters, &new_search_state);
        }
    }
    return false
}

fn search_to_the_bottom(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        if y + 1 < list_of_characters.len() {
            if x < list_of_characters[y + 1].len()
                && list_of_characters[y + 1].chars().nth(x) == Some(character_to_search)
            {
                let new_search_state: String = search_state.chars().skip(1).collect();
                return search_to_the_bottom(x, y + 1, list_of_characters, &new_search_state);
            }
        }
    }
    return false
}

fn search_to_the_top(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        if y > 0 {
            if x < list_of_characters[y - 1].len()
                && list_of_characters[y - 1].chars().nth(x) == Some(character_to_search)
            {
                let new_search_state: String = search_state.chars().skip(1).collect();
                return search_to_the_top(x, y - 1, list_of_characters, &new_search_state);
            }
        }
    }
    return false
}

fn search_to_the_bottom_right(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        if y + 1 < list_of_characters.len()
            && x + 1 < list_of_characters[y + 1].len()
            && list_of_characters[y + 1].chars().nth(x + 1) == Some(character_to_search)
        {
            let new_search_state: String = search_state.chars().skip(1).collect();
            return search_to_the_bottom_right(x + 1, y + 1, list_of_characters, &new_search_state);
        }
    }
    return false
}

fn search_to_the_bottom_left(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        if y + 1 < list_of_characters.len()
            && x > 0
            && list_of_characters[y + 1].chars().nth(x - 1) == Some(character_to_search)
        {
            let new_search_state: String = search_state.chars().skip(1).collect();
            return search_to_the_bottom_left(x - 1, y + 1, list_of_characters, &new_search_state);
        }
    }
    return false
}

fn search_to_the_top_right(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        if y > 0
            && x + 1 < list_of_characters[y - 1].len()
            && list_of_characters[y - 1].chars().nth(x + 1) == Some(character_to_search)
        {
            let new_search_state: String = search_state.chars().skip(1).collect();
            return search_to_the_top_right(x + 1, y - 1, list_of_characters, &new_search_state);
        }
    }
    return false
}

fn search_to_the_top_left(x: usize, y: usize, list_of_characters: &Vec<String>, search_state: &str) -> bool {
    if search_state.is_empty() {
        return true;
    }

    if let Some(character_to_search) = search_state.chars().next() {
        if y > 0
            && x > 0
            && list_of_characters[y - 1].chars().nth(x - 1) == Some(character_to_search)
        {
            let new_search_state: String = search_state.chars().skip(1).collect();
            return search_to_the_top_left(x - 1, y - 1, list_of_characters, &new_search_state);
        }
    }
    return false
}
