use std::collections::{HashSet, VecDeque};
use text_io::read;

enum Operands {Add, Mul, Concat}

#[derive(Debug, Clone)]
struct Operation {
    idx: usize,
    inter_res: i64,
    vals: VecDeque<i64>,
}

fn main() {
    let mut queue: VecDeque<Operation> = VecDeque::new();
    let mut seen: HashSet<usize> = HashSet::new();
    
    let mut idx = 0;
    let mut op_results: Vec<i64> = vec![];
    loop {
        let input: String = read!("{}\n");
        if input.is_empty() {
            break
        }
        let mut parts = input.split(":");
        op_results.push(parts.next().unwrap().parse().unwrap());
        let mut vals: VecDeque<i64> = parts.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();
        let inter_res = vals.pop_front().unwrap();
        queue.push_back(Operation { idx, inter_res, vals });
        idx += 1;
    }

    let mut result = 0;
    while !queue.is_empty() {
        let mut op = queue.pop_front().unwrap();
        if seen.contains(&op.idx) {
            continue;
        }
        if let Some(num) = op.vals.pop_front() {
            for operand in [Operands::Add, Operands::Mul, Operands::Concat].iter() {
                let mut new_op = op.clone();
                match operand {
                    Operands::Add => new_op.inter_res += num,
                    Operands::Mul => new_op.inter_res *= num,
                    Operands::Concat => new_op.inter_res = new_op.inter_res * 10_i64.pow(num.ilog10() + 1) + num,
                }
                if new_op.inter_res == op_results[new_op.idx] && new_op.vals.is_empty() && !seen.contains(&new_op.idx) {
                    result += new_op.inter_res;
                    seen.insert(new_op.idx);
                    break;
                } else if new_op.inter_res <= op_results[new_op.idx] {
                    queue.push_back(new_op);
                }
            }
        }
    }

    println!("Total calibration result: {result}");
}