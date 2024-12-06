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

| Command               | Description                                        |
|-----------------------|----------------------------------------------------|
| `./start_coding`       | *Fetches the input for the current day.*           |
| `./start_coding 12`    | *Fetches the input for the specified day (e.g., day 12).* |


- The input will be saved in `target/debug/input.txt` within the appropriate `dayX` directory.

## Structure  
Each day consists of two puzzles. The solutions for each day are organized into separate directories, named `dayX`, where `X` corresponds to the day number (e.g., `day1`, `day2`).**  

