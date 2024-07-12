use std::env;
use std::fs;
use std::process::{Command, exit};
use std::io::{self, Write};

fn print_help() {
    println!("gopen - A command-line tool to open Git repository remote URL in your browser");
    println!();
    println!("Usage:");
    println!("  gopen [DIRECTORY] [-i]");
    println!();
    println!("Options:");
    println!("  -h          Show this help message and exit");
    println!("  -i          Interactive mode to select remote URL");
    println!();
    println!("If DIRECTORY is not specified, the current directory is used.");
}

fn main() {
    // Set default directory to current directory
    let mut directory = String::from(".");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if -h parameter is present
    if args.contains(&String::from("-h")) {
        print_help();
        return;
    }

    // Check if a directory path is provided
    if args.len() > 1 && args[1] != "-i" {
        directory = args[1].clone();
    }

    // Check if the directory exists
    if !fs::metadata(&directory).is_ok() {
        eprintln!("Directory does not exist: {}", directory);
        exit(1);
    }

    // Get all remote URLs
    let output = Command::new("git")
        .arg("-C")
        .arg(&directory)
        .arg("remote")
        .arg("-v")
        .output()
        .expect("failed to execute git command");

    if !output.status.success() {
        eprintln!("Failed to get remote URL, please make sure you are running this script in a Git repository.");
        exit(1);
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let remote_urls: Vec<&str> = output_str
        .lines()
        .filter(|line| line.contains("(fetch)"))
        .map(|line| line.split_whitespace().nth(1).unwrap())
        .collect();

    // Check if any remote URLs were found
    if remote_urls.is_empty() {
        eprintln!("No remote URL found, please make sure you are running this script in a Git repository.");
        exit(1);
    }

    // Check if -i parameter is present
    let interactive = args.contains(&String::from("-i"));

    // If interactive mode, display selection menu
    let remote_url = if interactive {
        println!("Please select a remote URL:");
        for (i, url) in remote_urls.iter().enumerate() {
            println!("{}: {}", i + 1, url);
        }

        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("failed to read input");
        let index: usize = selection.trim().parse().expect("invalid input");

        if index == 0 || index > remote_urls.len() {
            eprintln!("Invalid selection, please try again.");
            exit(1);
        }

        remote_urls[index - 1]
    } else {
        // Default to the first remote URL
        remote_urls[0]
    };

    // Convert SSH URL to HTTPS URL if necessary
    let remote_url = if remote_url.starts_with("git@") {
        remote_url.replacen("git@", "https://", 1).replace(':', "/")
    } else if remote_url.starts_with("ssh://git@") {
        remote_url.replacen("ssh://git@", "https://", 1)
    } else {
        remote_url.to_string()
    };

    // Open the remote URL in the browser
    Command::new("open")
        .arg(remote_url)
        .status()
        .expect("failed to open URL");
}
