#![feature(let_chains)]

use std::io;
use std::io::Read;
use regex::Regex;

fn main() {
    
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    
    let mut input= String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    
    let mut result: i64 = 0;
    for matched in re.captures_iter(&*input) {
        if let (Some(num1), Some(num2)) = (matched.get(1), matched.get(2)) {
            let num1 = num1.as_str().parse::<i64>().unwrap();
            let num2 = num2.as_str().parse::<i64>().unwrap();
            result += num1 * num2;
        }  
    }
    println!("Sum of all multiplications: {}", result);
    
    let mut result: i64 = 0;
    let mut enabled = true;
    for matched in re.captures_iter(&*input) {
        if let (Some(num1), Some(num2)) = (matched.get(1), matched.get(2)) && enabled {
            let num1 = num1.as_str().parse::<i64>().unwrap();
            let num2 = num2.as_str().parse::<i64>().unwrap();
            result += num1 * num2;
        } else if matched.get(0).map_or(false, |m| m.as_str() == "do()") {
            enabled = true;
        } else if matched.get(0).map_or(false, |m| m.as_str() == "don't()") {
            enabled = false;
        }
    }
    
    println!("Sum of all multiplications with enable: {}", result);
}
