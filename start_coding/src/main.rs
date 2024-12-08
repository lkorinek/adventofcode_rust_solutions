use chrono::{Datelike, Local};
use clap::{arg, Parser};
use start_coding::{AutoInputScraper, Browsers};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};
use std::{fs, io};

/// A program that creates a Cargo package and downloads the current day's Advent of Code puzzle input
#[derive(Parser, Debug)]
#[command(version = "1.1", about)]
struct Args {
    /// Name of the file
    #[arg(short, long, default_value_t = String::from("input"))]
    file_name: String,

    /// Download just the input
    #[arg(long, default_value_t = false)]
    no_package: bool,

    /// Specifies the test mode (day is prefixed with 'test_')
    #[arg(short, long, default_value_t = false)]
    test: bool,

    /// Specifies the puzzle day
    #[arg(short, long)]
    day: Option<u32>,

    /// Specifies the browser to use
    #[arg(long = "browser", short = 'b', value_name = "BROWSER", value_enum, default_value_t = Browsers::Brave)]
    browser: Browsers,
}
const PACKAGE_NAME: &str = "day";

fn main() {
    let args = Args::parse();

    let now = Local::now();
    let day = if let Some(day) = args.day {
        day
    } else {
        now.day()
    };

    let new_package = format!(
        "{}{PACKAGE_NAME}{day}",
        if args.test { "test_" } else { "" }
    );
    if new_package.is_empty() {
        eprintln!("Error: Package name cannot be empty.");
        exit(1);
    }
    if !args.no_package {
        if !Path::new(&new_package).exists() {
            let status = Command::new("cargo")
                .arg("new")
                .arg(&new_package)
                .status()
                .expect("Failed to execute command");
            if status.success() {
                println!("Successfully created package '{}'", new_package);
            } else {
                eprintln!("Failed to create package '{}'", new_package);
                exit(1);
            }
        } else if should_continue(&format!(
            "The directory '{new_package}' already exists and will be removed."
        )) {
            if new_package.is_empty() {
                eprintln!("Error: Package name cannot be empty.");
                exit(1);
            }
            fs::remove_dir_all(&new_package).expect("Failed to remove existing directory");
            let status = Command::new("cargo")
                .arg("init")
                .arg(&new_package)
                .status()
                .expect("Failed to execute command");

            if status.success() {
                println!("Successfully initialized package '{}'", new_package);
            } else {
                eprintln!("Failed to initialize package '{}'", new_package);
                exit(1);
            }
        } else {
            exit(0);
        }
    }

    let scraper = AutoInputScraper::new();
    let alg_input = scraper.get_result(Some(day), args.browser);

    let dir = format!("{new_package}/");
    fs::create_dir_all(&dir).unwrap();
    let file_path = Path::new(&dir).join(format!("{}.txt", args.file_name));
    let mut file = File::create(file_path).unwrap();

    file.write_all(alg_input.as_bytes()).unwrap();
}

fn should_continue(extra: &str) -> bool {
    print!(
        "{}Do you want to continue? (y/N): ",
        if !extra.is_empty() {
            format!("{extra}\n")
        } else {
            "".to_owned()
        }
    );
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input.to_lowercase().as_str() {
        "y" | "Y" | "yes" | "Yes" => true,
        _ => false,
    }
}
