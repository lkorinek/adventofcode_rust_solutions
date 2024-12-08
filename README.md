# Advent of Code 2024 - Solutions  

Welcome to my **Advent of Code 2024** repository! 🎄✨  
This repository contains solutions written in Rust to the daily programming challenges presented in the [Advent of Code](https://adventofcode.com/2024) event.  

---

## About Advent of Code  
Advent of Code is an annual coding event consisting of daily programming puzzles from December 1st to December 25th. It’s a fun way to practice problem-solving, learn new algorithms, and enhance coding skills.  

---

## Advent of Code Automation

The `start_coding` folder automatically creates a Cargo package and downloads the input for the current day's Advent of Code puzzle (or a specified day) using your session cookies (from the Brave browser). The input is saved in `target/debug/input.txt` inside the corresponding `dayX` directory.

### Usage

| Command                | Description                                        |
|------------------------|----------------------------------------------------|
| `./start_coding`       | *Fetches the input for the current day.*           |
| `./start_coding -d 12` | *Fetches the input for the specified day (e.g., day 12).* |

### Options:

- `-f, --file-name <FILE_NAME>`  
  **Name of the file** where the puzzle input will be saved (default: `input(.txt)`).
  
- `--no-package`  
  **Download just the input** without creating a Cargo package.
  
- `-t, --test`  
  **Specifies the test mode** (the day's puzzle will be prefixed with `test_`).
  
- `-d, --day <DAY>`  
  **Specifies the puzzle day**. Provide a numeric value for the day (e.g., `-d 1` for Day 1).
  
- `-b, --browser <BROWSER>`  
  **Specifies the browser to use** for fetching the puzzle input. Possible values are:
  - `brave` (default)
  - `chrome`
  - `firefox`
  - `safari` (macos)

- `-h, --help`  
  **Print help** information about the program and its usage.

- `-V, --version`  
  **Print version** of the program.

### Example Usage

1. **Create a Cargo package and download the input for Day 1**:
    ```bash
    start_new_day -d 1
2. **Download input for Day 2, using Firefox as the browser**:
    ```bash
    start_new_day -d 2 -b firefox
3. **Download just the input for Day 3 without creating a package**:
    ```bash
    start_new_day --no-package -d 3

## Structure  
Each day consists of two puzzles. The solutions for each day are organized into separate directories, named `dayX`, where `X` corresponds to the day number (e.g., `day1`, `day2`).**  

