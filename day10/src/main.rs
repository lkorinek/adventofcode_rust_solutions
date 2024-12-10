use std::collections::{HashMap, HashSet, VecDeque};
use text_io::read;

#[derive(Default)]
struct Map {
    row_n: usize,
    col_n: usize,
    topography: Vec<Vec<usize>>,
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Map {

    fn move_in_dir(&mut self, pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
        let new_pos = (pos.0 as i32+dir.0, pos.1 as i32+dir.1);
        if self.is_in_bounds(new_pos) { Some((new_pos.0 as usize, new_pos.1 as usize)) } else { None }
    }

    fn get_starting_pos(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for row in 0..self.row_n {
            for col in 0..self.col_n {
                if self.topography[row][col] == 0 {
                    result.push((row, col));
                }
            }
        }
        result
    }

    fn find_trailheads(&mut self) -> usize {
        let starting_pos = self.get_starting_pos();
        let mut result = 0;
        for start in starting_pos {
            result += self.trailhead_bfs(start, false);
        }
        result
    }

    fn find_trailheads_all_traills(&mut self) -> usize {
        let starting_pos = self.get_starting_pos();
        let mut result = 0;
        for start in starting_pos {
            result += self.trailhead_bfs(start, true);
        }
        result
    }

    fn is_in_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.row_n as i32 && pos.1 < self.col_n as i32
    }

    fn is_trail(&mut self, pos: (usize, usize), dir: (i32, i32)) -> bool {
        if let Some(new_pos) = self.move_in_dir(pos, dir) {
            return self.topography[pos.0][pos.1] + 1 == self.topography[new_pos.0][new_pos.1]
        }
        false
    }

    fn find_neighbors(&mut self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        for dir in DIRS {
            if self.is_trail(pos, dir) {
                neighbors.push(self.move_in_dir(pos, dir).unwrap());
            }
        }
        neighbors
    }

    fn trailhead_bfs(&mut self, start: (usize, usize), visited_enabled: bool) -> usize {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(start);
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut trailheads: HashMap<(usize, usize), usize> = HashMap::new();
        while let Some(pos) = queue.pop_front() {
            // println!("Visited node: {:?}", pos);

            let neighbors = self.find_neighbors(pos);
            for &neighbor in &neighbors {
                if visited_enabled || !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    if self.topography[neighbor.0][neighbor.1] == 9{
                        let count = trailheads.entry(neighbor).or_insert(0);
                        *count += 1;
                    } else {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        trailheads.iter().map(|(_,&count)| if visited_enabled || count == 1 {count} else {0}).sum()
    }
}

fn main() {

    let mut map = Map::default();

    loop {
        let line: String = read!("{}\n");
        if line.is_empty() {
            break;
        }
        map.topography.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
        map.row_n += 1;
    }
    map.col_n = map.topography[0].len();

    println!("The trailheads count is: {}", map.find_trailheads());
    println!("The trailheads count of distinct hiking trails is: {}", map.find_trailheads_all_traills());
}
