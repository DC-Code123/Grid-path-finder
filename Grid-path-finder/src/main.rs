mod funcs;

use funcs::{Point};
use std::io::{self, Write};

fn main() {
    println!("=== Grid Path Finding Simulation ===\n");

    // Get grid limits
    println!("Enter row limit:");
    let row_limit = get_input("") as i32;
    println!("Enter col limit:");
    let col_limit = get_input("") as i32;

    // Generate start and destination points
    println!("\nGenerating start point...");
    let start = funcs::coordinate_generator(row_limit, col_limit);

    println!("\nGenerating destination point...");
    let destination = funcs::coordinate_generator(row_limit, col_limit);

    println!("\nStart: ({}, {})", start.row, start.col);
    println!("Destination: ({}, {})", destination.row, destination.col);

    // Generate obstacles
    let obstacles = funcs::random_absent_cell_selector(&start, &destination, row_limit, col_limit);
    println!("\nGenerated {} obstacles", obstacles.len());
    if obstacles.len() > 0 {
        println!("Obstacles at:");
        for (i, obs) in obstacles.iter().enumerate() {
            println!("  {}: ({}, {})", i + 1, obs.row, obs.col);
        }
    }

    println!("\nFinding path...\n");

    // Initialize path tracking
    let mut path: Vec<Point> = Vec::new();
    let mut current = start.clone();
    path.push(current.clone());

    // Calculate initial distance
    let mut distance = funcs::manhattan_distance(&current, &destination);

    // Main pathfinding loop
    let mut steps = 0;
    let max_steps = row_limit * col_limit * 2;

    while distance > 0 && steps < max_steps {
        steps += 1;

        // Get movement direction
        let (row_move, col_move, new_dist) = funcs::distance_calculator(&current, &destination);
        distance = new_dist;

        // Generate movement tag and display message
        let (tag, display) = funcs::movement_tag_generator(row_move, col_move);

        println!("Step {}: At ({}, {}) - {}", steps, current.row, current.col, display);

        // Calculate next potential position
        let next_point = funcs::movement_selector(&tag, &current);

        // Check if next position is valid
        if next_point.is_valid(row_limit, col_limit) {
            // Sensor and movement logic with obstacles
            if let Some(new_pos) = funcs::sensing_and_movement(
                &current,
                &next_point,
                &tag,
                &display,
                &mut path,
                row_limit,
                col_limit,
                &destination,
                &obstacles
            ) {
                current = new_pos;
            } else {
                println!("  No valid moves available! Stopping.");
                break;
            }
        } else {
            println!("  Movement would go out of bounds! Stopping.");
            break;
        }
    }

    // Display results
    println!("\n=== Path Finding Complete ===");
    if current == destination {
        println!("✓ Successfully reached destination in {} steps!", steps);
    } else {
        println!("✗ Stopped at ({}, {})", current.row, current.col);
        println!("  Destination: ({}, {})", destination.row, destination.col);
        if steps >= max_steps {
            println!("  Reason: Reached maximum step limit");
        }
    }

    println!("\nPath taken ({} steps):", path.len());
    for (i, point) in path.iter().enumerate() {
        println!("  {}: ({}, {})", i + 1, point.row, point.col);
    }
}

fn get_input(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            0
        }
    }
}
