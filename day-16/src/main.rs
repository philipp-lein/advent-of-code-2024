use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn all_directions() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

fn main() {
    // Read the maze from a file
    let maze: Vec<Vec<char>> = std::fs::read_to_string("input/input1.txt")
        .expect("Cannot read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Find the start (S) and end (E) points.
    let (start, end) = find_start_end(&maze);

    // Solve the maze.
    let (score, path) = solve_maze(&maze, start, end);

    // Output the result.
    println!("Minimum score to reach the end: {}", score);
    println!("Path: {:?}", path);
}

fn find_start_end(maze: &[Vec<char>]) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, row) in maze.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (i, j);
            } else if c == 'E' {
                end = (i, j);
            }
        }
    }
    return (start, end)
}

fn solve_maze(
    maze: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
) -> (usize, Vec<(usize, usize)>) {
    let mut queue = VecDeque::new();

    // Initialize: Each state includes the path, direction, and score
    for &initial_direction in &Direction::all_directions() {
        queue.push_back((vec![start], initial_direction, 1000)); // Path, Direction, Score
    }

    let mut best_score = usize::MAX;
    let mut best_path = Vec::new();
    let mut visited = std::collections::HashMap::new();

    while let Some((path, current_direction, score)) = queue.pop_front() {
        let &(row, col) = path.last().unwrap(); // Last position in the path

        // Prune paths with higher scores than the current best score
        if score >= best_score {
            continue;
        }

        // Check if we've reached the end
        if (row, col) == end {
            if score < best_score {
                best_score = score;
                best_path = path.clone();
            }
            continue; // Keep exploring other paths
        }

        // Avoid revisiting positions with worse scores
        let state_key = ((row, col), current_direction);
        if let Some(&best_known_score) = visited.get(&state_key) {
            if score >= best_known_score {
                continue;
            }
        }
        visited.insert(state_key, score);

        // Explore all directions
        for &new_direction in &Direction::all_directions() {
            let (drow, dcol) = new_direction.offset();
            let new_row = row as isize + drow;
            let new_col = col as isize + dcol;

            // Validate the new position
            if new_row >= 0
                && new_col >= 0
                && new_row < maze.len() as isize
                && new_col < maze[0].len() as isize
            {
                let new_pos = (new_row as usize, new_col as usize);

                // Only continue if the position is not a wall and not visited in this path
                if !path.contains(&new_pos) && maze[new_pos.0][new_pos.1] != '#' {
                    let mut new_path = path.clone();
                    new_path.push(new_pos);

                    // Calculate score: +1 for moving forward, +1000 for rotation if direction changes
                    let new_score = score
                        + 1
                        + if new_direction != current_direction { 
                            1000 
                        } else { 0 };

                    queue.push_back((new_path, new_direction, new_score));
                }
            }
        }
    }

    if best_score == usize::MAX {
        println!("No valid path found.");
    }
    return (best_score, best_path)
}

