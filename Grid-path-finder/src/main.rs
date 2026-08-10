mod funcs;

use std::io::{self, Write};
use funcs::{Point, display_grid, read_line, get_grid_limit, get_point, generate_obstacles, get_obstacle_count};

// ──────────── Main ────────────
fn main() {
    println!("=== Grid Path Finding Simulation ===\n");

    // Get grid limits with retry loop
    let (row_limit, col_limit) = loop {
        match get_grid_limit() {
            Ok(t) => break t,
            Err(e) => {
                eprint!("Error: {}. Retrying... ", e);
                io::stderr().flush().unwrap();
                // loop continues automatically
            }
        }
    };

    // Get start point
    let start = loop {
        match get_point("start", row_limit, col_limit) {
            Ok(p) => break p,
            Err(e) => {
                eprintln!("Error: {}. Please try again.", e);
                // retry
            }
        }
    };

    // Get destination point
    let destination = loop {
        match get_point("destination", row_limit, col_limit) {
            Ok(p) => {
                if p == start {
                    eprintln!("Destination cannot be the same as start. Try again.");
                    continue;
                }
                break p;
            }
            Err(e) => {
                eprintln!("Error: {}. Please try again.", e);
            }
        }
    };

    // Grid display choice
    let grid_display_choice = loop {
        let ans = read_line("Display grid after each step? (y/n): ")
            .unwrap_or_default().to_lowercase();
        if ans == "y" {
            println!("Grid will be displayed after each step.");
            break "y";
        } else if ans == "n" {
            println!("Grid will not be displayed after each step.");
            break "n";
        } else {
            eprintln!("Please answer 'y' or 'n'.");
        }
    };

    println!("\nStart: ({}, {})", start.row, start.col);
    println!("Destination: ({}, {})", destination.row, destination.col);

    // Generate obstacles (5-10)
    let obstacle_count = loop {
        match get_obstacle_count() {
            Ok(n) => break n,
            Err(e) => {
                eprintln!("Error: {}. Please try again.", e);
            }
        }
    };
    let obstacles = generate_obstacles(obstacle_count, &start, &destination, row_limit, col_limit);
    println!("\nGenerated {} obstacles", obstacles.len());
    if !obstacles.is_empty() {
        println!("Obstacles at:");
        for (i, obs) in obstacles.iter().enumerate() {
            println!("  {}: ({}, {})", i + 1, obs.row, obs.col);
        }
    }

    // ──────────── ALGORITHM SELECTION ────────────
    let algorithm = loop {
        let ans = read_line("\nSelect algorithm: (1) Greedy (reactive) (2) A* (optimal): ")
            .unwrap_or_default();
        match ans.trim() {
            "1" => break "greedy",
            "2" => break "astar",
            _ => eprintln!("Please enter 1 or 2."),
        }
    };
    println!();

    // ──────────── BRANCH 1: ORIGINAL GREEDY LOGIC ────────────
    if algorithm == "greedy" {
        println!("Running Greedy (reactive) algorithm...\n");

        let mut path: Vec<Point> = Vec::new();
        let mut current = start.clone();
        path.push(current.clone());

        if grid_display_choice == "y" {
            display_grid(row_limit, col_limit, &current, &destination, &obstacles, &path);
        }

        let mut distance = funcs::manhattan_distance(&current, &destination);
        let mut steps = 0;
        let max_steps = row_limit * col_limit * 2;

        while distance > 0 && steps < max_steps {
            steps += 1;
            let (row_move, col_move, new_dist) = funcs::distance_calculator(&current, &destination);
            distance = new_dist;
            let (tag, display) = funcs::movement_tag_generator(row_move, col_move);

            println!("Step {}: At ({}, {}) - {}", steps, current.row, current.col, display);

            let next_point = funcs::movement_selector(&tag, &current);
            if let Some(new_pos) = funcs::sensing_and_movement(
                &current, &next_point, &tag, &display, &mut path,
                row_limit, col_limit, &destination, &obstacles
            ) {
                current = new_pos;
                if grid_display_choice == "y" {
                    display_grid(row_limit, col_limit, &current, &destination, &obstacles, &path);
                }
            } else {
                println!("  No valid moves available! Stopping.");
                break;
            }
        }

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

    // ──────────── BRANCH 2: A* OPTIMAL LOGIC ────────────
    else {
        println!("Running A* (optimal) algorithm...\n");

        if let Some(full_path) = funcs::a_star_search(&start, &destination, &obstacles, row_limit, col_limit) {
            println!("✓ Optimal path found! Length: {} steps.", full_path.len() - 1);

            if grid_display_choice == "y" {
                println!("Animating path step-by-step...");
                for (i, point) in full_path.iter().enumerate() {
                    println!("Step {}: ({}, {})", i, point.row, point.col);
                    let partial_path = full_path[0..=i].to_vec();
                    display_grid(
                        row_limit,
                        col_limit,
                        point,
                        &destination,
                        &obstacles,
                        &partial_path,
                    );
                    // Optional: slow down animation (uncomment if desired)
                    // std::thread::sleep(std::time::Duration::from_millis(200));
                }
            } else {
                // Show final grid with the whole path
                display_grid(
                    row_limit,
                    col_limit,
                    &destination,
                    &destination,
                    &obstacles,
                    &full_path,
                );
            }

            println!("\n=== Path Finding Complete ===");
            println!("✓ Successfully reached destination in {} steps!", full_path.len() - 1);

            println!("\nPath taken ({} steps):", full_path.len());
            for (i, point) in full_path.iter().enumerate() {
                println!("  {}: ({}, {})", i + 1, point.row, point.col);
            }
        } else {
            println!("✗ No valid path exists between start and destination!");
            display_grid(
                row_limit,
                col_limit,
                &start,
                &destination,
                &obstacles,
                &vec![start.clone()],
            );
        }
    }
}