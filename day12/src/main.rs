use std::collections::{HashSet, VecDeque};
use text_io::read;

struct Plot {
    kind: char,
    checked: bool,
    sides: HashSet<(i32, i32)>,
}

#[derive(Default)]
struct Garden {
    row_n: usize,
    col_n: usize,
    map: Vec<Vec<Plot>>,
}

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

impl Garden {

    fn move_in_dir(&mut self, pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
        let new_pos = (pos.0 as i32+dir.0, pos.1 as i32+dir.1);
        if self.is_in_bounds(new_pos) { Some((new_pos.0 as usize, new_pos.1 as usize)) } else { None }
    }

    fn is_in_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.row_n as i32 && pos.1 < self.col_n as i32
    }

    fn is_same_type(&mut self, pos0: (usize, usize), pos1: (usize, usize)) -> bool {
        self.map[pos0.0][pos0.1].kind == self.map[pos1.0][pos1.1].kind
    }

    fn get_free_sides(&self, pos: (usize, usize), dir: (i32, i32)) -> HashSet<(i32, i32)> {
        let ver: [(i32, i32); 2] = [(1, 0), (-1, 0)];
        let hor: [(i32, i32); 2] = [(0, 1), (0, -1)];
        if ver.contains(&dir) {
            self.map[pos.0][pos.1]
                .sides
                .iter()
                .cloned()
                .filter(|side| hor.contains(side))
                .collect()
        } else {
            self.map[pos.0][pos.1]
                .sides
                .iter()
                .cloned()
                .filter(|side| ver.contains(side))
                .collect()
        }
    }

    fn find_neighbors_and_sides(&mut self, pos: (usize, usize), allow_free: bool) -> (Vec<(usize, usize)>, usize) {
        let mut neighbors: Vec<(usize, usize)> = vec![];
        let mut free_sides: HashSet<(i32, i32)> = HashSet::new();
        for dir in DIRS {
            if let Some(new_pos) = self.move_in_dir(pos, dir) {
                if self.is_same_type(pos, new_pos) && self.is_visited(new_pos) {
                    free_sides.extend(self.get_free_sides(new_pos, dir));
                }
            }
        }

        for dir in DIRS {
            if let Some(new_pos) = self.move_in_dir(pos, dir) {
                if self.is_same_type(pos, new_pos) {
                    if !self.is_visited(new_pos) {
                        neighbors.push(new_pos);
                    }
                } else {
                    self.map[pos.0][pos.1].sides.insert(dir);
                }
            } else {
                self.map[pos.0][pos.1].sides.insert(dir);
            }
        }

        let sides_n =
        if allow_free {
            self.map[pos.0][pos.1]
                .sides
                .iter()
                .filter(|side| !free_sides.contains(side))
                .count()
        } else { self.map[pos.0][pos.1].sides.len() };

        (neighbors, sides_n)
    }

    fn set_visited(&mut self, pos: (usize, usize)) {
        self.map[pos.0][pos.1].checked = true;
    }

    fn is_visited(&mut self, pos: (usize, usize)) -> bool {
        self.map[pos.0][pos.1].checked
    }

    fn region_bfs(&mut self, start: (usize, usize), allow_free: bool) -> (usize, usize) {
        self.set_visited(start);
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut perimeter: usize = 0;
        let mut area: usize = 0;
        while let Some(pos) = queue.pop_front() {
            let (neighbors, sides_n) = self.find_neighbors_and_sides(pos, allow_free);
            perimeter += sides_n;
            area += 1;
            for neighbor in neighbors {
                if !self.is_visited(neighbor) {
                    self.set_visited(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        (perimeter, area)
    }

    fn get_price_region(&mut self, pos: (usize, usize), allow_free: bool) -> usize {
        let (perimeter, area) = self.region_bfs(pos, allow_free);
        perimeter * area
    }

    fn get_total_price(&mut self, allow_free: bool) -> usize {
        let mut total_price = 0;
        for row in 0..self.row_n {
            for col in 0..self.col_n {
                let pos = (row, col);
                if !self.is_visited(pos) {
                    total_price += self.get_price_region(pos, allow_free);
                }
            }
        }
        total_price
    }
}

fn main() {

    let mut garden = Garden::default();

    loop {
        let line: String = read!("{}\n");

        if line.is_empty() {
            break;
        }

        garden.row_n += 1;
        garden.map.push(line
            .chars()
            .map(|kind| Plot{kind, checked: false, sides: HashSet::new()})
            .collect());
    }
    garden.col_n = garden.map.get(0).expect("No input given").len();

    println!("the total price is: {}", garden.get_total_price(false));

    for row in &mut garden.map {
        for plot in row {
            plot.sides = HashSet::new();
            plot.checked = false;
        }
    }
    println!("the total price with discount is: {}", garden.get_total_price(true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_price() {
        let input: &str = "AAAA
BBCD
BBCC
EEEC";

        let mut garden = Garden::default();

        for line in input.lines() {
            garden.row_n += 1;
            garden.map.push(line
                .chars()
                .map(|kind| Plot{kind, checked: false, sides: HashSet::new()})
                .collect());
        }
        garden.col_n = garden.map.get(0).expect("No input given").len();

        assert_eq!(140, garden.get_total_price(false));
    }

    #[test]
    fn test_total_price_discount() {
        let input: &str = "AAAA
BBCD
BBCC
EEEC";

        let mut garden = Garden::default();

        for line in input.lines() {
            garden.row_n += 1;
            garden.map.push(line
                .chars()
                .map(|kind| Plot{kind, checked: false, sides: HashSet::new()})
                .collect());
        }
        garden.col_n = garden.map.get(0).expect("No input given").len();

        assert_eq!(80, garden.get_total_price(true));
    }
}
