mod funcs;

use funcs::{Point, display_grid};
use std::io::{self, Write};

fn main() {
    println!("=== Grid Path Finding Simulation ===\n");

    // Get grid limits
    println!("Enter row limit:");
    let row_limit = get_input("") as i32;
    println!("Enter col limit:");
    let col_limit = get_input("") as i32;

    // Choice to randomly generate start/destination or input manually
    // The start point:
    println!("\nDo you want to randomly generate the start point? (y/n):");
    let start_choice = get_input_string("").to_lowercase();
    let start = if start_choice == "y" {
        funcs::coordinate_generator(row_limit, col_limit)
    } else {
        println!("Enter start row:");
        let row = get_input("") as i32;
        println!("Enter start col:");
        let col = get_input("") as i32;
        Point::new(row, col)
    };
    // The destination point:
    println!("\nDo you want to randomly generate the destination point? (y/n):");
    let dest_choice = get_input_string("").to_lowercase();
    let destination = if dest_choice == "y" {
        funcs::coordinate_generator(row_limit, col_limit)
    } else {
        println!("Enter destination row:");
        let row = get_input("") as i32;
        println!("Enter destination col:");
        let col = get_input("") as i32;
        Point::new(row, col)
    };

    // The grid display choice:
    println!("\nDo you wish to display the grid after each step? (y/n):");
    let grid_display_choice = get_input_string("").to_lowercase();
    if grid_display_choice == "y" {
        println!("Grid will be displayed after each step.");
        let grid_display_choice: bool = true;
    } else {
        println!("Grid will not be displayed after each step.");
        let grid_display_choice: bool = false;
    }



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

    // Show initial grid
    println!("Initial grid:");
    if grid_display_choice == true {
        display_grid(row_limit, col_limit, &current, &destination, &obstacles, &path);
    }

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

        // Sensor and movement logic with obstacles and bounds
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
            if grid_display_choice == true {
                display_grid(row_limit, col_limit, &current, &destination, &obstacles, &path);
            }
        } else {
            println!("  No valid moves available! Stopping.");
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

fn get_input_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input
}