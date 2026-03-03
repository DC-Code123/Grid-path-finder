mod funcs;

use funcs::{Coordinate, MovementTag, MovementDisplay};
use std::io::{self, Write};

fn main() {
    println!("=== Path Finding Simulation ===\n");
    
    // Get grid dimensions
    let grid_rows = get_input("Enter number of grid rows: ");
    let grid_cols = get_input("Enter number of grid columns: ");
    
    // Generate start and destination coordinates
    let start = Coordinate::new(
        get_input("Enter start row (0-based): "),
        get_input("Enter start col (0-based): ")
    );
    
    let destination = Coordinate::new(
        get_input("Enter destination row (0-based): "),
        get_input("Enter destination col (0-based): ")
    );
    
    // Validate coordinates
    if !start.is_valid(grid_rows, grid_cols) || !destination.is_valid(grid_rows, grid_cols) {
        println!("Error: Coordinates out of grid bounds!");
        return;
    }
    
    println!("\nStart: {:?}", start);
    println!("Destination: {:?}", destination);
    println!("Finding path...\n");
    
    // Initialize path tracking
    let mut path: Vec<Coordinate> = Vec::new();
    let mut current = start.clone();
    
    // Calculate initial Manhattan distance
    let mut dist = funcs::manhattan_distance(&current, &destination);
    
    // Main pathfinding loop
    while dist > 0 {
        // Get movement direction
        let (rad, cad, new_dist) = funcs::calculate_direction(&current, &destination);
        dist = new_dist;
        
        // Generate movement tag and display message
        let (tag, display) = funcs::generate_movement_tag(rad, cad);
        
        println!("Current position: {:?} - {}", current, display);
        
        // Calculate next potential position
        let next_pos = funcs::apply_movement(&tag, &current);
        
        // Check if next position is valid
        if next_pos.is_valid(grid_rows, grid_cols) {
            // Sensor and movement logic
            if let Some(new_pos) = funcs::sensor_and_movement(
                &current,
                &next_pos,
                &tag,
                &display,
                &mut path,
                grid_rows,
                grid_cols
            ) {
                current = new_pos;
            } else {
                // Path blocked, try alternatives
                println!("  Path blocked, recalculating...");
            }
        } else {
            println!("  Movement would go out of bounds!");
            break;
        }
        
        // Safety check to prevent infinite loops
        if path.len() > grid_rows * grid_cols * 2 {
            println!("Path too long, stopping to prevent infinite loop");
            break;
        }
    }
    
    // Display results
    println!("\n=== Path Finding Complete ===");
    if current == destination {
        println!("Successfully reached destination!");
    } else {
        println!("Stopped at {:?}", current);
        println!("Destination: {:?}", destination);
    }
    
    println!("\nPath taken ({} steps):", path.len());
    for (i, coord) in path.iter().enumerate() {
        println!("  {}: {:?}", i + 1, coord);
    }
}

fn get_input(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    input.trim().parse().expect("Please enter a valid number")
}