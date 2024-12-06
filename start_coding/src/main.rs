use chrono::{Datelike, Local};
use start_coding::AutoInputScraper;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};
use std::{env, fs};

const PACKAGE_NAME: &str = "day";

fn main() {
    let args: Vec<String> = env::args().collect();

    let now = Local::now();
    let day: u32 = match args.len() {
        1 => now.day(),
        _ => {
            let arg = &args[1]; // First argument is the day of the month
            arg.parse().expect("Invalid number of the day.")
        }
    };

    let new_package = format!("{PACKAGE_NAME}{day}");
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

    let scraper = AutoInputScraper::new();
    let alg_input = scraper.get_result(Some(day));

    let dir = format!("{new_package}/target/debug");
    fs::create_dir_all(&dir).unwrap();
    let file_path = Path::new(&dir).join("input.txt");
    let mut file = File::create(file_path).unwrap();

    file.write_all(alg_input.as_bytes()).unwrap();
}
