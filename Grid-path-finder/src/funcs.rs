use rand::Rng;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn new(row: i32, col: i32) -> Self {
        Point { row, col }
    }

    pub fn is_valid(&self, max_rows: i32, max_cols: i32) -> bool {
        self.row >= 0 && self.row < max_rows && self.col >= 0 && self.col < max_cols
    }
}

#[derive(Debug, PartialEq)]
pub enum MovementTag {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    Non,
}

/// Generate random coordinate within grid limits
pub fn coordinate_generator(row_limit: i32, col_limit: i32) -> Point {
    let mut rng = rand::thread_rng();
    Point::new(rng.gen_range(0..row_limit), rng.gen_range(0..col_limit))
}

/// Generate movement tag based on direction components
pub fn movement_tag_generator(row_move: i32, col_move: i32) -> (MovementTag, String) {
    match (row_move, col_move) {
        (1, 1) => (MovementTag::DownRight, "C moves down to the right".to_string()),
        (1, 0) => (MovementTag::Down, "C moves down".to_string()),
        (1, -1) => (MovementTag::DownLeft, "C moves down to the left".to_string()),

        (-1, 1) => (
            MovementTag::UpRight,
            "C moves up to the right".to_string(),
        ),
        (-1, 0) => (MovementTag::Up, "C moves up".to_string()),
        (-1, -1) => (
            MovementTag::UpLeft,
            "C moves up to the left".to_string(),
        ),

        (0, 1) => (MovementTag::Right, "C moves right".to_string()),
        (0, -1) => (MovementTag::Left, "C moves left".to_string()),
        (0, 0) => (MovementTag::Non, "C doesn't move".to_string()),

        _ => (MovementTag::Non, "Invalid movement".to_string()),
    }
}

/// Apply movement tag to get new point
pub fn movement_selector(tag: &MovementTag, current: &Point) -> Point {
    match tag {
        MovementTag::Up => Point::new(current.row - 1, current.col),
        MovementTag::Down => Point::new(current.row + 1, current.col),
        MovementTag::Left => Point::new(current.row, current.col - 1),
        MovementTag::Right => Point::new(current.row, current.col + 1),
        MovementTag::UpLeft => Point::new(current.row - 1, current.col - 1),
        MovementTag::UpRight => Point::new(current.row - 1, current.col + 1),
        MovementTag::DownLeft => Point::new(current.row + 1, current.col - 1),
        MovementTag::DownRight => Point::new(current.row + 1, current.col + 1),
        MovementTag::Non => current.clone(),
    }

}

/// Calculate direction components between two points
pub fn distance_calculator(current: &Point, destination: &Point) -> (i32, i32, i32) {
    let row_move = (destination.row - current.row).signum();
    let col_move = (destination.col - current.col).signum();
    let absolute_dist = manhattan_distance(current, destination);
    (row_move, col_move, absolute_dist)
}

/// Calculate Manhattan distance between two points
pub fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.row - b.row).abs() + (a.col - b.col).abs()
}

/// Check if a point is present (simulated sensor)
pub fn presence_checker(_next_point: &Point) -> bool {
    // In a real implementation, this would communicate with the point
    // For simulation, we assume all points are present unless they're obstacles
    true
}

/// Generate random obstacles
pub fn random_absent_cell_selector(
    start_point: &Point,
    destination_point: &Point,
    row_limit: i32,
    col_limit: i32,
) -> Vec<Point> {
    println!("How many obstacles do you wish?");
    print!("Enter number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number_obstacles: i32 = input.trim().parse().unwrap_or(0);

    let mut obstacles = Vec::new();
    let mut i = 0;
    let mut rng = rand::thread_rng();

    while i < number_obstacles {
        let cell = Point::new(rng.gen_range(0..row_limit), rng.gen_range(0..col_limit));

        if cell != *start_point && cell != *destination_point && !obstacles.contains(&cell) {
            obstacles.push(cell);
            i += 1;
        }
    }

    obstacles
}

/// Main sensor and movement logic
pub fn sensing_and_movement(
    current: &Point,
    next_point: &Point,
    tag: &MovementTag,
    display: &str,
    path: &mut Vec<Point>,
    grid_rows: i32,
    grid_cols: i32,
    destination: &Point,
    obstacles: &Vec<Point>,
) -> Option<Point> {
    println!(
        "  Sensor checking position ({}, {})...",
        next_point.row, next_point.col
    );

    // Check if next point is present and not an obstacle
    let mut next_point_presence = next_point.is_valid(grid_rows, grid_cols) &&
        presence_checker(next_point) &&
        !obstacles.contains(next_point);
    if obstacles.contains(next_point) {
        next_point_presence = false;
    }

    if next_point_presence {
        println!("  ✓ {} - Position is available", display);
        if !path.contains(next_point) {
            path.push(next_point.clone());
        }
        Some(next_point.clone())
    } else {
        println!("  ✗ {} - Position is blocked!", display);
        // Try alternative options
        movement_options_selector(
            current,
            tag,
            path,
            grid_rows,
            grid_cols,
            destination,
            obstacles,
        )
    }
}

/// Select alternative movement when primary path is blocked
fn movement_options_selector(
    current: &Point,
    tag: &MovementTag,
    path: &mut Vec<Point>,
    grid_rows: i32,
    grid_cols: i32,
    destination: &Point,
    obstacles: &Vec<Point>,
) -> Option<Point> {
    println!("  Searching for alternative path...");

    // Get alternative positions based on current tag
    let (possible_point_1, possible_point_2) = match tag {
        MovementTag::Right => (
            Point::new(current.row - 1, current.col + 1),
            Point::new(current.row + 1, current.col + 1),
        ),
        MovementTag::Left => (
            Point::new(current.row - 1, current.col - 1),
            Point::new(current.row + 1, current.col - 1),
        ),
        MovementTag::Up => (
            Point::new(current.row - 1, current.col - 1),
            Point::new(current.row - 1, current.col + 1),
        ),
        MovementTag::Down => (
            Point::new(current.row + 1, current.col + 1),
            Point::new(current.row + 1, current.col - 1),
        ),
        MovementTag::UpRight => (
            Point::new(current.row - 1, current.col),
            Point::new(current.row, current.col + 1),
        ),
        MovementTag::DownRight => (
            Point::new(current.row + 1, current.col),
            Point::new(current.row, current.col + 1),
        ),
        MovementTag::UpLeft => (
            Point::new(current.row - 1, current.col),
            Point::new(current.row, current.col - 1),
        ),
        MovementTag::DownLeft => (
            Point::new(current.row + 1, current.col),
            Point::new(current.row, current.col - 1),
        ),
        MovementTag::Non => return None,
    };

    movement(
        &possible_point_1,
        &possible_point_2,
        current,
        path,
        grid_rows,
        grid_cols,
        destination,
        obstacles,
    )
}

/// Evaluate and choose between two possible movement points
fn movement(
    possible_point_1: &Point,
    possible_point_2: &Point,
    current_point: &Point,
    path: &mut Vec<Point>,
    grid_rows: i32,
    grid_cols: i32,
    destination: &Point,
    obstacles: &Vec<Point>,
) -> Option<Point> {
    // Check presence for both possible points
    let mut possible_point_1_presence = presence_checker(possible_point_1);
    let mut possible_point_2_presence = presence_checker(possible_point_2);

    if obstacles.contains(possible_point_1) {
        possible_point_1_presence = false;
    }
    if obstacles.contains(possible_point_2) {
        possible_point_2_presence = false;
    }

    // Check validity
    let point1_valid = possible_point_1.is_valid(grid_rows, grid_cols);
    let point2_valid = possible_point_2.is_valid(grid_rows, grid_cols);

    match (
        point1_valid && possible_point_1_presence,
        point2_valid && possible_point_2_presence,
    ) {
        (true, true) => {
            // Both points available - choose the one closer to destination
            let dist1 = manhattan_distance(possible_point_1, destination);
            let dist2 = manhattan_distance(possible_point_2, destination);

            let chosen = if dist1 <= dist2 {
                possible_point_1
            } else {
                possible_point_2
            };
            println!(
                "  Both alternatives available, choosing closer to destination: ({}, {})",
                chosen.row, chosen.col
            );

            if !path.contains(chosen) {
                path.push(chosen.clone());
            }
            Some(chosen.clone())
        }
        (true, false) => {
            println!(
                "  Found alternative: ({}, {})",
                possible_point_1.row, possible_point_1.col
            );
            if !path.contains(possible_point_1) {
                path.push(possible_point_1.clone());
            }
            Some(possible_point_1.clone())
        }
        (false, true) => {
            println!(
                "  Found alternative: ({}, {})",
                possible_point_2.row, possible_point_2.col
            );
            if !path.contains(possible_point_2) {
                path.push(possible_point_2.clone());
            }
            Some(possible_point_2.clone())
        }
        (false, false) => {
            println!("  No alternatives found in primary direction");
            full_presence_checker(
                current_point,
                destination,
                path,
                grid_rows,
                grid_cols,
                obstacles,
            )
        }
    }
}

/// Check all surrounding cells when no alternatives in primary direction
fn full_presence_checker(
    point: &Point,
    destination: &Point,
    path: &mut Vec<Point>,
    grid_rows: i32,
    grid_cols: i32,
    obstacles: &Vec<Point>,
) -> Option<Point> {
    // Generate all 8 surrounding cells
    let surrounding_cells = vec![
        Point::new(point.row - 1, point.col),     // Up
        Point::new(point.row - 1, point.col + 1), // Up-Right
        Point::new(point.row, point.col + 1),     // Right
        Point::new(point.row + 1, point.col + 1), // Down-Right
        Point::new(point.row + 1, point.col),     // Down
        Point::new(point.row + 1, point.col - 1), // Down-Left
        Point::new(point.row, point.col - 1),     // Left
        Point::new(point.row - 1, point.col - 1), // Up-Left
    ];

    // Filter available cells (valid, present, not in path, not obstacles)
    let mut available_cells: Vec<Point> = surrounding_cells
        .into_iter()
        .filter(|cell| cell.is_valid(grid_rows, grid_cols))
        .filter(|cell| presence_checker(cell))
        .filter(|cell| !obstacles.contains(cell))
        .filter(|cell| !path.contains(cell))
        .collect();

    if !available_cells.is_empty() {
        // Find the cell closest to destination
        available_cells.sort_by(|a, b| {
            let dist_a = manhattan_distance(a, destination);
            let dist_b = manhattan_distance(b, destination);
            dist_a.cmp(&dist_b)
        });

        let best_cell = available_cells[0].clone();
        println!(
            "  Found available cell from surroundings: ({}, {})",
            best_cell.row, best_cell.col
        );

        if !path.contains(&best_cell) {
            path.push(best_cell.clone());
        }
        Some(best_cell)
    } else {
        println!("  No available cells found anywhere!");
        None
    }
}

//ASCII grid display of current state(for debugging and visualization easily)
pub fn display_grid(
    rows: i32,
    cols: i32,
    current: &Point,
    destination: &Point,
    obstacles: &Vec<Point>,
    path: &Vec<Point>,
) {
    // Display legend:
    // '.' = empty cell
    // '#' = obstacle
    // 'S' = start position
    // 'D' = destination position
    // 'C' = current position
    // '*' = visited path cell
    let rows = rows as usize;
    let cols = cols as usize;
    let mut grid = vec![vec!['.'; cols]; rows];

    for obstacle in obstacles {
        if obstacle.is_valid(rows as i32, cols as i32) {
            grid[obstacle.row as usize][obstacle.col as usize] = '#';
        }
    }

    if let Some(start) = path.first() {
        if start.is_valid(rows as i32, cols as i32) {
            grid[start.row as usize][start.col as usize] = 'S';
        }
    }

    for point in path.iter().skip(1) {
        if point.is_valid(rows as i32, cols as i32)
            && grid[point.row as usize][point.col as usize] == '.'
        {
            grid[point.row as usize][point.col as usize] = '*';
        }
    }

    if destination.is_valid(rows as i32, cols as i32) {
        grid[destination.row as usize][destination.col as usize] = 'D';
    }

    if current.is_valid(rows as i32, cols as i32) {
        grid[current.row as usize][current.col as usize] = 'C';
    }

    println!("\nGrid (row 0 at top):");
    print!("   ");
    for col in 0..cols {
        print!("{:2}", col);
    }
    println!();

    for row in 0..rows {
        print!("{:2} ", row);
        for col in 0..cols {
            print!(" {}", grid[row][col]);
        }
        println!();
    }
}
