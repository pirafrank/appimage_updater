// src/main.rs
use clap::Parser;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::process::Command;

#[derive(Parser)]
struct Args {
    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    j: usize,
}

fn main() {
    let args = Args::parse();

    // Check if appimageupdatetool is installed
    if Command::new("appimageupdatetool").output().is_err() {
        eprintln!("Error: appimageupdatetool not found. Install it and try again. Aborting.");
        std::process::exit(1);
    }

    // Get all directories in $PATH
    let path_var = env::var("PATH").expect("Failed to get PATH environment variable");
    let paths: Vec<&str> = path_var.split(':').collect();

    // Collect all AppImage files
    let mut appimages = Vec::new();
    for path in paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext.eq_ignore_ascii_case("appimage") {
                            appimages.push(path);
                        }
                    }
                }
            }
        }
    }

    // Run appimageupdatetool in parallel
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.j)
        .build_global()
        .unwrap();

    appimages.par_iter().for_each(|appimage| {
        println!("\n\nUpdating {:?}", appimage);
        Command::new("appimageupdatetool")
            .arg("-O")
            .arg(appimage)
            .status()
            .expect("Failed to execute appimageupdatetool");
    });
}
