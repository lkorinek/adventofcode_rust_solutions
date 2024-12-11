use std::collections::HashMap;
use text_io::read;

fn main() {

    let line: String = read!("{}\n");
    let stones: Vec<usize> = line
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut stones_map: HashMap<usize, usize> = HashMap::new();
    for &stone in &stones {
        let count = stones_map.entry(stone).or_insert(0);
        *count += 1;
    }

    for i in 0..75 {

        let mut new_stones_map: HashMap<usize, usize> = HashMap::new();

        stones_map.iter().for_each(|(&stone, &count)| {
            match stone {
                0 => {
                    *new_stones_map.entry(stone + 1).or_insert(0) += count;
                }
                _ if (stone.ilog10() + 1) % 2 == 0 => {
                    let splitter = 10_usize.pow((stone.ilog10() + 1) / 2);
                    let left_stone = stone / splitter;
                    let right_stone= stone % splitter;

                    *new_stones_map.entry(left_stone).or_insert(0) += count;
                    *new_stones_map.entry(right_stone).or_insert(0) += count;
                }
                _ => {
                    *new_stones_map.entry(stone * 2024).or_insert(0) += count;
                }
            }
        });


        stones_map = new_stones_map;
        if i + 1 == 25 {
            let total_count: usize = stones_map.values().sum();
            println!("After blinking {} times the total count is: {}", i+1, total_count);
        }
    }

    let total_count: usize = stones_map.values().sum();
    println!("After blinking 75 times the total count is: {total_count}");
}
