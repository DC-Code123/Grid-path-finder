use rand::Rng;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
        (1, 1) => (MovementTag::DownRight, "C moves downwards towards the right".to_string()),
        (1, 0) => (MovementTag::Down, "C moves down".to_string()),
        (1, -1) => (MovementTag::DownLeft, "C moves downwards towards the left".to_string()),

        (-1, 1) => (
            MovementTag::UpRight,
            "C moves upward towards the right".to_string(),
        ),
        (-1, 0) => (MovementTag::Up, "C moves up".to_string()),
        (-1, -1) => (
            MovementTag::UpLeft,
            "C moves upwards towards the left".to_string(),
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

// ──────────── Pure parsing functions (improvement #7) ────────────
fn parse_i32(input: &str) -> Result<i32, String> {
    input.parse::<i32>()
        .map_err(|_| format!("'{}' is not a valid integer", input)) // #2 specific message
}

fn parse_point(row_str: &str, col_str: &str) -> Result<Point, String> {
    let row = parse_i32(row_str)?;
    let col = parse_i32(col_str)?;
    Ok(Point::new(row, col))
}

// ──────────── Improved input with EOF handling (#1) ────────────
pub fn read_line(prompt: &str) -> Option<String> {
    print!("{}", prompt);
    io::stdout().flush().ok()?;
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(0) => None,   // Ctrl+D / EOF
        Ok(_) => Some(input.trim().to_string()),
        Err(_) => None,
    }
}

// ──────────── Grid limits with defaults, retry, cancellation (#3, #5, #6, #9) ────────────
pub fn get_grid_limit() -> Result<(i32, i32), String> {
    const MAX_LIMIT: i32 = 100_000;   // #6 sensible maximum

    // Ask random or manual – uses the same y/n logic
    let random_choice = loop {
        let ans = read_line("Randomly generate row & col limits? (y/n): ")
            .unwrap_or_default()
            .to_lowercase();
        if ans == "y" || ans == "n" { break ans == "y"; }
        eprintln!("Please answer 'y' or 'n'.");
    };

    if random_choice {
        let mut rng = rand::thread_rng();
        let rows = rng.gen_range(5..=50);
        let cols = rng.gen_range(5..=50);
        println!("Randomly generated: rows = {}, cols = {}", rows, cols);
        return Ok((rows, cols));
    }

    let mut attempts = 0;
    loop {
        // #5: default 10 when Enter is pressed
        let row_str = read_line("Enter row limit (default 10): ").unwrap_or_default();
        let row_str = if row_str.is_empty() { "10".to_string() } else { row_str };

        let col_str = read_line("Enter col limit (default 10): ").unwrap_or_default();
        let col_str = if col_str.is_empty() { "10".to_string() } else { col_str };

        // #7: parse using pure function
        let row_limit = match parse_i32(&row_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: {}", e);
                attempts += 1;
                if attempts >= 3 {  // #3: retry cancellation
                    let keep = read_line("Keep trying? (y/n): ").unwrap_or_default().to_lowercase();
                    if keep != "y" {
                        return Err("User cancelled grid entry.".to_string());
                    }
                    attempts = 0;
                }
                sleep(Duration::from_millis(500)); // #9
                continue;
            }
        };
        let col_limit = match parse_i32(&col_str) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: {}", e);
                attempts += 1;
                if attempts >= 3 {
                    let keep = read_line("Keep trying? (y/n): ").unwrap_or_default().to_lowercase();
                    if keep != "y" {
                        return Err("User cancelled grid entry.".to_string());
                    }
                    attempts = 0;
                }
                sleep(Duration::from_millis(500));
                continue;
            }
        };

        // #6 max limit check
        if row_limit > MAX_LIMIT || col_limit > MAX_LIMIT {
            eprintln!("Error: limit too large (max {}). Got ({}, {})", MAX_LIMIT, row_limit, col_limit);
            attempts += 1;
            if attempts >= 3 {
                let keep = read_line("Keep trying? (y/n): ").unwrap_or_default().to_lowercase();
                if keep != "y" { return Err("User cancelled grid entry.".to_string()); }
                attempts = 0;
            }
            sleep(Duration::from_millis(500));
            continue;
        }

        if row_limit > 0 && col_limit > 0 {
            return Ok((row_limit, col_limit));
        } else {
            eprintln!("Error: limits must be positive.");
            attempts += 1;
            if attempts >= 3 {
                let keep = read_line("Keep trying? (y/n): ").unwrap_or_default().to_lowercase();
                if keep != "y" { return Err("User cancelled grid entry.".to_string()); }
                attempts = 0;
            }
            sleep(Duration::from_millis(500));
        }
    }
}

// ──────────── Point input (start / destination) with random option ────────────
pub fn get_point(label: &str, row_limit: i32, col_limit: i32) -> Result<Point, String> {
    let choice = loop {
        let ans = read_line(&format!("Randomly generate {} point? (y/n): ", label))
            .unwrap_or_default().to_lowercase();
        if ans == "y" || ans == "n" { break ans == "y"; }
        eprintln!("Please answer 'y' or 'n'.");
    };

    if choice {
        Ok(coordinate_generator(row_limit, col_limit))
    } else {
        let mut attempts = 0;
        loop {
            let row_str = read_line(&format!("Enter {} row: ", label)).unwrap_or_default();
            let col_str = read_line(&format!("Enter {} col: ", label)).unwrap_or_default();

            let point = parse_point(&row_str, &col_str)?;
            if point.row < 0 || point.row >= row_limit || point.col < 0 || point.col >= col_limit {
                eprintln!("Error: point must be within 0..{} for row and 0..{} for col.", row_limit-1, col_limit-1);
                attempts += 1;
                if attempts >= 3 {
                    let keep = read_line("Keep trying? (y/n): ").unwrap_or_default().to_lowercase();
                    if keep != "y" { return Err("User cancelled point entry.".to_string()); }
                    attempts = 0;
                }
                sleep(Duration::from_millis(500));
                continue;
            }
            return Ok(point);
        }
    }
}

// ──────────── Obstacle count input (default 5-10 random) ────────────
pub fn get_obstacle_count() -> Result<usize, String> {
    let input = read_line("Enter number of obstacles (or press Enter for random 5-10): ")
        .unwrap_or_default();
    if input.is_empty() {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(5..=10);
        println!("Randomly chosen obstacle count: {}", n);
        return Ok(n);
    }
    let n = parse_i32(&input)?;
    if n <= 0 {
        return Err("Obstacle count must be greater than 0.".to_string());
    }
    Ok(n as usize)
}

// ──────────── Obstacle generator (uses given count) ────────────
pub fn generate_obstacles(count: usize, start: &Point, dest: &Point, rows: i32, cols: i32) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let mut obstacles = Vec::new();
    let max_possible = (rows * cols) as usize - 2; // exclude start & dest
    let actual_count = count.min(max_possible);

    while obstacles.len() < actual_count {
        let r = rng.gen_range(0..rows);
        let c = rng.gen_range(0..cols);
        let candidate = Point::new(r, c);
        if candidate != *start && candidate != *dest && !obstacles.contains(&candidate) {
            obstacles.push(candidate);
        }
    }
    obstacles
}

// ──────────── A* Search (optimal pathfinding) ────────────
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone)]
struct AStarNode {
    point: Point,
    g: i32,  // cost from start
    h: i32,  // heuristic
    f: i32,  // g + h
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order so BinaryHeap becomes a min‑heap on `f`
        other.f.cmp(&self.f)
            .then_with(|| self.g.cmp(&other.g))
    }
}
impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Heuristic: Chebyshev distance (allows 8‑directional moves with cost 1)
fn heuristic_estimate(a: &Point, b: &Point) -> i32 {
    (a.row - b.row).abs().max((a.col - b.col).abs())
}

/// Returns all 8 valid neighbouring cells
fn get_neighbors(point: &Point, rows: i32, cols: i32) -> Vec<Point> {
    let mut neighbors = Vec::with_capacity(8);
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; }
            let candidate = Point::new(point.row + dr, point.col + dc);
            if candidate.is_valid(rows, cols) {
                neighbors.push(candidate);
            }
        }
    }
    neighbors
}

/// Reconstruct path from the `came_from` map
fn reconstruct_path(mut came_from: HashMap<Point, Point>, goal: &Point) -> Vec<Point> {
    let mut path = Vec::new();
    let mut current = goal.clone();
    while let Some(prev) = came_from.get(&current) {
        path.push(current.clone());
        current = prev.clone();
    }
    path.push(current);
    path.reverse();
    path
}

/// Public A* function – returns the shortest path (in steps) if one exists
pub fn a_star_search(
    start: &Point,
    goal: &Point,
    obstacles: &[Point],
    rows: i32,
    cols: i32,
) -> Option<Vec<Point>> {
    if start == goal {
        return Some(vec![start.clone()]);
    }
    if obstacles.contains(start) || obstacles.contains(goal) {
        return None;
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut g_score: HashMap<Point, i32> = HashMap::new();
    let mut closed_set: HashSet<Point> = HashSet::new();

    let start_h = heuristic_estimate(start, goal);
    g_score.insert(start.clone(), 0);
    open_set.push(AStarNode {
        point: start.clone(),
        g: 0,
        h: start_h,
        f: start_h,
    });

    while let Some(current_node) = open_set.pop() {
        let current = current_node.point;

        if current == *goal {
            return Some(reconstruct_path(came_from, goal));
        }

        if closed_set.contains(&current) {
            continue;
        }
        closed_set.insert(current.clone());

        let current_g = *g_score.get(&current).unwrap_or(&i32::MAX);

        for neighbor in get_neighbors(&current, rows, cols) {
            if obstacles.contains(&neighbor) || closed_set.contains(&neighbor) {
                continue;
            }

            // Uniform cost of 1 for any of the 8 directions
            let tentative_g = current_g + 1;

            if tentative_g < *g_score.get(&neighbor).unwrap_or(&i32::MAX) {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g);
                let h = heuristic_estimate(&neighbor, goal);
                let f = tentative_g + h;
                open_set.push(AStarNode {
                    point: neighbor.clone(),
                    g: tentative_g,
                    h,
                    f,
                });
            }
        }
    }

    None // No path found
}