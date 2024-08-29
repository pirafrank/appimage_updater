// src/main.rs
use clap::Parser;
use colored::*;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Parser)]
struct Args {
    /// Number of threads to use (1-8)
    #[arg(short, long, default_value_t = 4)]
    j: usize,
    /// Run appimageupdatetool to only check for updates
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,
    /// Show version information
    #[arg(short, long)]
    version: bool,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
const COMMIT: &str = env!("GIT_COMMIT_HASH");
const BUILD_DATE: &str = env!("BUILD_DATE");

const APPIMAGEUPDATETOOL: &str = "appimageupdatetool";

fn main() {
    let args = Args::parse();

    // Show version information and exit
    if args.version {
        println!("Version: {}\nCommit: {}\nBuild Date: {}", VERSION.white(), COMMIT.white(), BUILD_DATE.white());
        std::process::exit(0);
    }

    // Check if the number of threads is within the allowed range
    if args.j < 1 || args.j > 8 {
        eprintln!("{}", "Error: Number of threads must be between 1 and 8. Aborting.".red());
        std::process::exit(1);
    }

    // Check if appimageupdatetool is installed
    if Command::new(APPIMAGEUPDATETOOL).output().is_err() {
        eprintln!("{}", format!("Error: {} not found. Install it and try again. Aborting.", APPIMAGEUPDATETOOL).red());
        std::process::exit(1);
    }

    // if dry-run mode is enabled, use the -j as argument for appimageupdatetool instead of -O.
    let apparg = if args.dry_run { "-j" } else { "-O" };
    if args.dry_run {
        eprintln!("{}", format!(
          "\n** Dry-run mode enabled. {} will check for updates but won't apply them. **\n",
          APPIMAGEUPDATETOOL
        ).yellow());
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

    // Define colors for up to 8 threads, excluding yellow and using red only for errors
    let colors = [
        Color::Green,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::BrightGreen,
        Color::BrightBlue,
        Color::BrightMagenta,
    ];

    // Run appimageupdatetool in parallel
    let counter = Arc::new(AtomicUsize::new(1));
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.j)
        .build_global()
        .unwrap();

    appimages.par_iter().for_each(|appimage| {
        let thread_num = counter.fetch_add(1, Ordering::SeqCst);
        let color = colors[(thread_num - 1) % colors.len()];
        println!("{}", format!("[Thread {}] Updating {:?}", thread_num, appimage).color(color));
        let output = Command::new(APPIMAGEUPDATETOOL)
            .arg(apparg)
            .arg(appimage)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect(&format!("Failed to execute {}", APPIMAGEUPDATETOOL))
            .wait_with_output()
            .expect("Failed to wait on child");

        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);

        for line in stdout_str.lines() {
            println!("{}", format!("[Thread {}] {}", thread_num, line).color(color));
        }

        for line in stderr_str.lines() {
            println!("{}", format!("[Thread {}] {}", thread_num, line).color(color));
        }

        if !output.status.success() {
            eprintln!("{}", format!("[Thread {}] Error: Failed to update {:?}", thread_num, appimage).red());
            if let Some(code) = output.status.code() {
                // check if the exit code is 2 as per the appimageupdatetool source code:
                // - describe function:
                //   https://github.com/AppImageCommunity/AppImageUpdate/blob/5e91de84aba775ba8d3a4771e4f7f06056f9b764/src/cli/main.cpp#L154
                // - update function:
                //   https://github.com/AppImageCommunity/AppImageUpdate/blob/5e91de84aba775ba8d3a4771e4f7f06056f9b764/src/cli/main.cpp#L177
                if code == 2 {
                    eprintln!("{}", format!("[Thread {}] This is likely an AppImage issue. \
                    For more info, try running: {} -d {:?}", thread_num, APPIMAGEUPDATETOOL, appimage).red());
                }
            }
        }
    });
}
