use std::io::Read;

use tempfile::TempDir;
use serde_json::to_string_pretty;

use crate::{project::Project};

mod interpreter;
mod sprite;
mod thread;
mod project;

fn main() {
    let mut project = Project::new(get_project_file_path()).expect("Could not load project");

    /*if let Ok(entries) = std::fs::read_dir(project.path) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.path().display());
            }
        }
    } else {
        eprintln!("Failed to read directory contents");
    }*/

    let starting_time = std::time::Instant::now();
    project.run();
    println!(
        "RASH:\n{} seconds elapsed, 1 million iterations",
        starting_time.elapsed().as_secs_f64()
    );

    let starting_time = std::time::Instant::now();
    let pi = calculate_pi();
    println!(
        "\nRUST:\n{} seconds elapsed, 1 million iterations\n\npi = {}",
        starting_time.elapsed().as_secs_f64(),
        pi
    );
}

fn get_project_file_path() -> String {
    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if an argument (file path) is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Extract the file path from the arguments
    let file_path = &args[1];
    file_path.clone()
}

fn calculate_pi() -> f64 {
    let mut pi = 0.0;
    let n = 4.0;
    let mut d = 1.0;
    for i in 0..10_000_000 {
        let a = (2 * (i % 2) - 1) as f64;
        pi += a * n / d;
        d += 2.0;
    }
    pi
}
