use std::collections::HashMap;
use text_io::read;

const DATA_LENGTH: u32 = 1000;

fn main() {
    let mut list1: Vec<i64> = vec![];
    let mut list2: Vec<i64> = vec![];

    for _ in 0..DATA_LENGTH {
        let input: String = read!("{}\n");
        let mut parts = input.split_whitespace();
        list1.push(parts.next().unwrap().parse().expect("Invalid input"));
        list2.push(parts.next().unwrap().parse().expect("Invalid input"));
    }

    list1.sort();
    list2.sort();

    let total_distance: i64 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Distance: {total_distance}");


    let mut counter: HashMap<i64, i64> = HashMap::new();

    for &num in &list2 {
        let count = counter.entry(num).or_insert(0);
        *count += 1;
    }

    let similarity_score: i64 = list1
        .iter()
        .map(|x| x * counter.get(x).unwrap_or(&0))
        .sum();

    println!("Similarity score: {similarity_score}");
}
