use regex::Regex;
use text_io::read;

const ROW_N: isize = 103;
const COL_N: isize = 101;
const SECS: isize = 100;

fn is_candidate(grid: &Vec<Vec<i32>>) -> bool {
    for row in grid {
        let mut count = 0;
        for &value in row {
            if value > 0 {
                count += 1;
                if count > 7 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
    }
    false
}

fn get_robot_pos(robot: &Vec<isize>, sec: isize) -> (isize, isize) {
    let pos = (robot[0], robot[1]);
    let vel = (robot[2], robot[3]);
    let mut new_pos = ((pos.0 + vel.0*sec)%COL_N, (pos.1 + vel.1*sec)%ROW_N);
    if new_pos.0 < 0 {
        new_pos.0 = COL_N + new_pos.0;
    }
    if new_pos.1 < 0 {
        new_pos.1 = ROW_N + new_pos.1;
    }
    new_pos
}
fn draw_grid(grid: &Vec<Vec<i32>>, iteration: isize) {
    println!("Iteration: {}\n", iteration);

    for row in grid {
        for &cell in row {
            print!("{}", if cell > 0 {cell.to_string()} else {".".to_string()});
        }
        println!();
    }
}

fn main() {
    let mut quadrant_counts = [0,0,0,0];
    let mut robots: Vec<Vec<isize>> = vec![];

    loop {
        let robot_info: String = read!{"{}\n"};
        if robot_info.is_empty() {
            break;
        }
        let re = Regex::new(r"(-)?\d+").unwrap();

        let robot: Vec<isize> = re
            .captures_iter(&*robot_info)
            .filter_map(|cap| {
                cap.get(0)
                    .map(|m| m.as_str().parse::<isize>().ok())
                    .flatten()
            })
            .collect();

        if robot.len() != 4 {
            panic!("Invalid input provided!");
        }

        let new_pos = get_robot_pos(&robot, SECS);
        let col_idx = if new_pos.0 < COL_N/2 {
            0
        } else if new_pos.0 != COL_N/2 {
            1
        } else {
            continue
        };
        let row_idx = if new_pos.1 < ROW_N/2 {
            0
        } else if new_pos.1 != ROW_N/2 {
            1
        } else {
            continue
        };

        robots.push(robot);
        quadrant_counts[row_idx*2+col_idx] += 1;
    }

    println!("The safety factor is: {}", quadrant_counts.iter().product::<usize>());

    let mut sec = 1;
    loop {
        let mut grid: Vec<Vec<i32>> = vec![vec![0; COL_N as usize]; ROW_N as usize];
        for robot in &robots {
            let new_pos = get_robot_pos(robot, sec);
            grid[new_pos.1 as usize][new_pos.0 as usize] += 1;
        }

        if is_candidate(&grid) {
            draw_grid(&grid, sec);
        }
        sec += 1;
    }
}
