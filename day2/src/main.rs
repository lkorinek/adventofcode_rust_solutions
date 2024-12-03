use text_io::read;

const DATA_LENGTH: u32 = 1000;

fn main() {
    
    let mut safe_count = 0;
    let mut safe_one_tolerated_count = 0;
    for _ in 0..DATA_LENGTH {
        let input: String = read!("{}\n");
        let parts = input.split_whitespace();
        let level: Vec<i64> = parts.map(|x| x.parse::<i64>().unwrap()).collect();
        if is_safe(&level) {
            safe_count += 1;
        }
        if is_safe_one_tolerated(&level) {
            safe_one_tolerated_count += 1;
        }
    }
    
    println!("Safe levels: {safe_count}");
    println!("Safe levels with one bad level tolerated: {safe_one_tolerated_count}");
}

fn is_safe(level: &Vec<i64>) -> bool {
    let mut inc = true;
    for idx in 0..level.len()-1 {
        let diff = level[idx+1] - level[idx];
        if idx == 0 && diff < 0 {
            inc = false;
        }
        match inc {
            true => {
                if diff < 1 || diff > 3 {
                    return false
                }
            }
            false => { 
                if diff < -3 || diff > -1 {
                    return false
                }
            }
        }
    }
    true
}

fn is_safe_one_tolerated(level: &Vec<i64>) -> bool {
    for idx in 0..level.len() {
        let mut tmp_level = level.clone();
        tmp_level.remove(idx);
        if is_safe(&tmp_level) {
            return true
        }
    }
    false
}
