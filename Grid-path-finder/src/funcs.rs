use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

impl Coordinate {
    pub fn new(row: i32, col: i32) -> Self {
        Coordinate { row, col }
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
    UpRightDiag,
    UpLeftDiag,
    DownRightDiag,
    DownLeftDiag,
    NoMove,
}

pub struct MovementDisplay {
    pub tag: MovementTag,
    pub description: String,
}

/// Calculate direction components between two coordinates
pub fn calculate_direction(current: &Coordinate, destination: &Coordinate) -> (i32, i32, i32) {
    let rad = (destination.row - current.row).signum();
    let cad = (destination.col - current.col).signum();
    let distance = manhattan_distance(current, destination);
    (rad, cad, distance)
}

/// Calculate Manhattan distance between two coordinates
pub fn manhattan_distance(a: &Coordinate, b: &Coordinate) -> i32 {
    (a.row - b.row).abs() + (a.col - b.col).abs()
}

/// Generate movement tag based on direction components
pub fn generate_movement_tag(rad: i32, cad: i32) -> (MovementTag, String) {
    match (rad, cad) {
        (1, 1) => (MovementTag::UpRightDiag, "C moves to the upward right diagonal".to_string()),
        (1, 0) => (MovementTag::Up, "C moves upward".to_string()),
        (1, -1) => (MovementTag::UpLeftDiag, "C moves to the upward left diagonal".to_string()),
        
        (-1, 1) => (MovementTag::DownRightDiag, "C moves to the downward right diagonal".to_string()),
        (-1, 0) => (MovementTag::Down, "C moves downward".to_string()),
        (-1, -1) => (MovementTag::DownLeftDiag, "C moves to the left downward diagonal".to_string()),
        
        (0, 1) => (MovementTag::Right, "C moves to the right".to_string()),
        (0, -1) => (MovementTag::Left, "C moves to the left".to_string()),
        (0, 0) => (MovementTag::NoMove, "We are here".to_string()),
        
        _ => (MovementTag::NoMove, "Invalid movement".to_string()),
    }
}

/// Apply movement tag to get new coordinate
pub fn apply_movement(tag: &MovementTag, current: &Coordinate) -> Coordinate {
    match tag {
        MovementTag::Up => Coordinate::new(current.row - 1, current.col),
        MovementTag::Down => Coordinate::new(current.row + 1, current.col),
        MovementTag::Left => Coordinate::new(current.row, current.col - 1),
        MovementTag::Right => Coordinate::new(current.row, current.col + 1),
        MovementTag::UpLeftDiag => Coordinate::new(current.row - 1, current.col - 1),
        MovementTag::UpRightDiag => Coordinate::new(current.row - 1, current.col + 1),
        MovementTag::DownLeftDiag => Coordinate::new(current.row + 1, current.col - 1),
        MovementTag::DownRightDiag => Coordinate::new(current.row + 1, current.col + 1),
        MovementTag::NoMove => current.clone(),
    }
}

/// Check if a position is present (simulated sensor)
pub fn check_presence(position: &Coordinate, grid_rows: i32, grid_cols: i32) -> bool {
    // This is a simplified presence check
    // In a real implementation, this might check against obstacles or other agents
    position.is_valid(grid_rows, grid_cols) && (position.row + position.col) % 3 != 0
}

/// Main sensor and movement logic
pub fn sensor_and_movement(
    current: &Coordinate,
    next_pos: &Coordinate,
    tag: &MovementTag,
    display: &str,
    path: &mut Vec<Coordinate>,
    grid_rows: i32,
    grid_cols: i32,
) -> Option<Coordinate> {
    println!("  Sensor checking position {:?}...", next_pos);
    
    // Check if the next position is present/available
    let is_present = check_presence(next_pos, grid_rows, grid_cols);
    
    if is_present {
        println!("  {} - Position is available", display);
        path.push(next_pos.clone());
        Some(next_pos.clone())
    } else {
        println!("  {} - Position is blocked!", display);
        // Try alternative options
        select_alternative_movement(current, tag, path, grid_rows, grid_cols)
    }
}

/// Select alternative movement when primary path is blocked
fn select_alternative_movement(
    current: &Coordinate,
    tag: &MovementTag,
    path: &mut Vec<Coordinate>,
    grid_rows: i32,
    grid_cols: i32,
) -> Option<Coordinate> {
    println!("  Searching for alternative path...");
    
    // Get alternative positions based on current tag
    let alternatives = match tag {
        MovementTag::Right => vec![
            Coordinate::new(current.row - 1, current.col + 1),
            Coordinate::new(current.row + 1, current.col + 1),
        ],
        MovementTag::Left => vec![
            Coordinate::new(current.row - 1, current.col - 1),
            Coordinate::new(current.row + 1, current.col - 1),
        ],
        MovementTag::Up => vec![
            Coordinate::new(current.row - 1, current.col + 1),
            Coordinate::new(current.row - 1, current.col - 1),
        ],
        MovementTag::Down => vec![
            Coordinate::new(current.row + 1, current.col + 1),
            Coordinate::new(current.row + 1, current.col - 1),
        ],
        MovementTag::UpRightDiag => vec![
            Coordinate::new(current.row - 1, current.col),
            Coordinate::new(current.row, current.col + 1),
        ],
        MovementTag::DownRightDiag => vec![
            Coordinate::new(current.row + 1, current.col),
            Coordinate::new(current.row, current.col + 1),
        ],
        MovementTag::UpLeftDiag => vec![
            Coordinate::new(current.row - 1, current.col),
            Coordinate::new(current.row, current.col - 1),
        ],
        MovementTag::DownLeftDiag => vec![
            Coordinate::new(current.row + 1, current.col),
            Coordinate::new(current.row, current.col - 1),
        ],
        MovementTag::NoMove => vec![],
    };
    
    // Filter valid alternatives and check presence
    let valid_alternatives: Vec<Coordinate> = alternatives
        .into_iter()
        .filter(|pos| pos.is_valid(grid_rows, grid_cols))
        .filter(|pos| check_presence(pos, grid_rows, grid_cols))
        .collect();
    
    match valid_alternatives.len() {
        0 => {
            // If no alternatives, check all surrounding cells
            select_from_surrounding(current, path, grid_rows, grid_cols)
        }
        1 => {
            println!("  Found alternative: {:?}", valid_alternatives[0]);
            path.push(valid_alternatives[0].clone());
            Some(valid_alternatives[0].clone())
        }
        _ => {
            // Choose the closest alternative to destination
            // This would require destination info - simplified for now
            println!("  Multiple alternatives available, choosing first: {:?}", valid_alternatives[0]);
            path.push(valid_alternatives[0].clone());
            Some(valid_alternatives[0].clone())
        }
    }
}

/// Select from all surrounding cells when no alternatives in primary direction
fn select_from_surrounding(
    current: &Coordinate,
    path: &mut Vec<Coordinate>,
    grid_rows: i32,
    grid_cols: i32,
) -> Option<Coordinate> {
    // Generate all 8 surrounding cells
    let surrounding = vec![
        Coordinate::new(current.row - 1, current.col),     // Up
        Coordinate::new(current.row - 1, current.col + 1), // Up-Right
        Coordinate::new(current.row, current.col + 1),     // Right
        Coordinate::new(current.row + 1, current.col + 1), // Down-Right
        Coordinate::new(current.row + 1, current.col),     // Down
        Coordinate::new(current.row + 1, current.col - 1), // Down-Left
        Coordinate::new(current.row, current.col - 1),     // Left
        Coordinate::new(current.row - 1, current.col - 1), // Up-Left
    ];
    
    // Filter valid and present cells
    let available: Vec<Coordinate> = surrounding
        .into_iter()
        .filter(|pos| pos.is_valid(grid_rows, grid_cols))
        .filter(|pos| check_presence(pos, grid_rows, grid_cols))
        .collect();
    
    if !available.is_empty() {
        // Sort by some heuristic (simplified - just take first)
        println!("  Found available cell from surroundings: {:?}", available[0]);
        path.push(available[0].clone());
        Some(available[0].clone())
    } else {
        println!("  No available cells found!");
        None
    }
}