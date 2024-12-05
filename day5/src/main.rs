use text_io::read;

const RULES_LENGTH: usize = 1176;
const UPDATES_LENGTH: usize = 1385 - 1177;

fn main() {
    let mut rules: [Vec<usize>; 100] = std::array::from_fn(|_| Vec::new());
    
    for _ in 0..RULES_LENGTH {
        let input: String = read!("{}\n");
        let rule_nums: Vec<usize> = input
            .split('|')
            .map(|s| s.trim().parse().unwrap()) 
            .collect();
        rules[rule_nums[0]].push(rule_nums[1]);
    }
    
    let _: String = read!("{}\n");
    let mut result = (0, 0);
    
    for _ in 0..UPDATES_LENGTH {

        let input: String = read!("{}\n");
        let mut nums: Vec<usize> = input
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let mut is_correct = true;
        let mut idx = 0;
        let mut present = [false; 100];

        'outer: while idx < nums.len() {
            present[nums[idx]] = true;
            for &successor in &rules[nums[idx]] { 
                if present[successor] {
                    let successor_idx = nums.iter().position(|&x| x == successor).unwrap();
                    nums[successor_idx] = nums[idx];
                    nums[idx] = successor;
                    present = [false; 100];
                    idx = 0;
                    is_correct = false;
                    continue 'outer;
                }
            }
            idx += 1;
        }
    
        if is_correct {
            result.0 += nums[nums.len() / 2];
        } else {
            result.1 += nums[nums.len() / 2];
        }
    }

    println!("Middle numbers sum: {}", result.0);
    println!("Middle corrected numbers sum: {}", result.1);
}
