use std::fs;

fn main() {
    let (game_map, movements) = read_input("input/input1.txt");
    let result_map = solve(&game_map, &movements);
    for row in &result_map {
        println!("{}", row.iter().map(|s| s.as_str()).collect::<String>());
    }

    let gps_coordinates = calculate_gps_coordinates(&result_map);
    println!("GPS Coordinates: {:?}", gps_coordinates);
    println!("Answer: {}", gps_coordinates.iter().sum::<i32>());
}

fn read_input(file_path: &str) -> (Vec<Vec<String>>, String) {
    let input = fs::read_to_string(file_path).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let game_map_lines = parts[0];
    let movements_lines = parts[1];
    let game_map: Vec<Vec<String>> = game_map_lines
        .lines()
        .filter(|line| !line.trim().is_empty()) // Remove empty lines
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    let movements = movements_lines.replace("\n", ""); // Remove newlines in movements

    (game_map, movements)
}

fn solve(game_map: &Vec<Vec<String>>, movements: &String) -> Vec<Vec<String>> {
    let mut map = game_map.clone();
    let mut robot_position = (0, 0);

    // Locate the robot's initial position
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == "@" {
                robot_position = (i, j);
            }
        }
    }

    // Process movements
    for movement in movements.chars() {
        let (dx, dy) = match movement {
            '^' => (-1, 0), // Move up
            'v' => (1, 0),  // Move down
            '<' => (0, -1), // Move left
            '>' => (0, 1),  // Move right
            _ => continue,  // Ignore invalid movements
        };

        let new_position = (
            (robot_position.0 as isize + dx) as usize,
            (robot_position.1 as isize + dy) as usize,
        );

        if let Some(cell) = map
            .get(new_position.0)
            .and_then(|row| row.get(new_position.1))
        {
            match cell.as_str() {
                "#" => continue, // Wall: no movement
                "O" => {
                    // Attempt to push the line of boxes
                    if can_move_boxes(&map, new_position, dx, dy) {
                        move_all_boxes(&mut map, new_position, dx, dy);
                        // Move the robot
                        map[robot_position.0][robot_position.1] = ".".to_string();
                        map[new_position.0][new_position.1] = "@".to_string();
                        robot_position = new_position;
                    }
                }
                _ => {
                    // Empty space: move the robot
                    map[new_position.0][new_position.1] = "@".to_string();
                    map[robot_position.0][robot_position.1] = ".".to_string();
                    robot_position = new_position;
                }
            }
        }
    }

    map
}

fn can_move_boxes(
    game_map: &Vec<Vec<String>>,
    mut box_position: (usize, usize),
    dx: isize,
    dy: isize,
) -> bool {
    loop {
        let next_position = (box_position.0 as isize + dx, box_position.1 as isize + dy);

        let next_cell = &game_map[next_position.0 as usize][next_position.1 as usize];
        match next_cell.as_str() {
            "#" => return false, // Wall blocks the path
            "O" => box_position = (next_position.0 as usize, next_position.1 as usize), // Continue checking
            _ => return true, // Empty space allows movement
        }
    }
}

fn move_all_boxes(
    game_map: &mut Vec<Vec<String>>,
    box_position: (usize, usize),
    dx: isize,
    dy: isize,
) {
    let mut positions_to_update = Vec::new();
    let mut current_position = box_position;

    // Collect all positions to update
    loop {
        let next_position = (
            current_position.0 as isize + dx,
            current_position.1 as isize + dy,
        );

        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= game_map.len() as isize
            || next_position.1 >= game_map[0].len() as isize
        {
            break;
        }

        let next_cell = &game_map[next_position.0 as usize][next_position.1 as usize];
        if next_cell == "." {
            // Found an empty cell, stop here
            positions_to_update.push((
                current_position,
                (next_position.0 as usize, next_position.1 as usize),
            ));
            break;
        } else if next_cell == "O" {
            // Continue moving boxes
            positions_to_update.push((
                current_position,
                (next_position.0 as usize, next_position.1 as usize),
            ));
            current_position = (next_position.0 as usize, next_position.1 as usize);
        } else {
            break;
        }
    }

    // Update positions
    for (from, to) in positions_to_update.iter().rev() {
        game_map[to.0][to.1] = "O".to_string();
        game_map[from.0][from.1] = ".".to_string();
    }
}

fn calculate_gps_coordinates(game_map: &Vec<Vec<String>>) -> Vec<i32> {
    let mut gps_coordinates = Vec::new();

    for (i, row) in game_map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == "O" {
                let gps_coordinate = (100 * i as i32) + j as i32;
                gps_coordinates.push(gps_coordinate);
            }
        }
    }

    return gps_coordinates;
}
