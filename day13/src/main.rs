use good_lp::{constraint, default_solver, Solution, SolverModel, ProblemVariables, variable};
use regex::Regex;
use text_io::read;

fn get_algebra_solution(const_a: (isize, isize, isize), const_b: (isize, isize, isize)) -> Option<(usize, usize)> {
    let det = const_a.0*const_b.1-const_a.1*const_b.0;
    if det == 0 {
        return None;
    }
    let a = (const_b.1*const_a.2-const_a.1*const_b.2) as f64 /det as f64;
    let b = (const_b.2*const_a.0-const_a.2*const_b.0) as f64 /det as f64;
    if a != a.trunc() || b != b.trunc() {
        return None;
    }

    Some((a as usize, b as usize))
}

fn get_lp_solution(const_a: (i32, i32, i32), const_b: (i32, i32, i32)) -> Option<(i32, i32)> {
    let mut problem = ProblemVariables::new();
    let a = problem.add(variable().integer().min(0).max(100));
    let b = problem.add(variable().integer().min(0).max(100));
    let solution = problem.minimise(3 * a + b)
        .using(default_solver)
        .with(constraint!(const_a.0 * a + const_a.1 * b <= const_a.2))
        .with(constraint!(const_a.0 * a + const_a.1 * b >= const_a.2))
        .with(constraint!(const_b.0 * a + const_b.1 * b <= const_b.2))
        .with(constraint!(const_b.0 * a + const_b.1 * b >= const_b.2))
        .solve()
        .ok()?;
    Some((solution.value(a).round() as i32, solution.value(b).round() as i32))
}

fn parse_coordinates(text: &str, regex: &Regex) -> Option<(i32, i32)> {
    regex.captures(text).map(|cap| {
        let x = cap.get(1).and_then(|m| m.as_str().parse::<i32>().ok()).unwrap_or(0);
        let y = cap.get(2).and_then(|m| m.as_str().parse::<i32>().ok()).unwrap_or(0);
        (x, y)
    })
}

fn main() {
    let mut tokens_total_n = 0;
    let mut tokens_total_n_converted = 0;
    loop {
        let button_a: String = read!{"{}\n"};
        if button_a.is_empty() {
            break;
        }
        let button_b: String = read!{"{}\n"};
        let price: String = read!{"{}\n"};
        let _: String = read!{"{}\n"};

        let re = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let re_price = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

        let mut const_a: (i32, i32, i32) = (0,0,0);
        let mut const_b: (i32, i32, i32) = (0,0,0);
        if let Some((x1, y1)) = parse_coordinates(&button_a, &re) {
            const_a.0 = x1;
            const_b.0 = y1;
        }

        if let Some((x2, y2)) = parse_coordinates(&button_b, &re) {
            const_a.1 = x2;
            const_b.1 = y2;
        }

        if let Some((px, py)) = parse_coordinates(&price, &re_price) {
            const_a.2 = px;
            const_b.2 = py;
        }

        if let Some(solution) = get_lp_solution(const_a, const_b) {
            tokens_total_n += 3*solution.0 + solution.1;
        }
        let mut const_a = (const_a.0 as isize, const_a.1 as isize, const_a.2 as isize);
        let mut const_b = (const_b.0 as isize, const_b.1 as isize, const_b.2 as isize);
        const_a.2 += 10000000000000;
        const_b.2 += 10000000000000;
        if let Some(solution) = get_algebra_solution(const_a, const_b) {
            tokens_total_n_converted += 3*solution.0 + solution.1;
        }
    }

    println!("The smallest number of tokens is: {tokens_total_n}");
    println!("The smallest number of tokens with conversion is: {tokens_total_n_converted}");
}