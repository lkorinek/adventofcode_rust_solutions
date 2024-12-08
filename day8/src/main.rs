use std::collections::{HashMap, HashSet};
use text_io::read;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Pos(isize, isize);

#[derive(Default)]
struct Map {
    grid: Vec<Vec<char>>,
    antennas: HashMap<char, Vec<Pos>>,
    row_n: usize,
    col_n: usize,
    max_dist: Option<usize>,
}

impl Map {
    fn find_antennas(&mut self) {
        for row in 0..self.row_n {
            for col in 0..self.col_n {
                if self.grid[row][col] != '.' {
                    let antenna = self.grid[row][col];
                    if let Some(vec) = self.antennas.get_mut(&antenna) {
                        vec.push(Pos(row as isize, col as isize));
                    } else {
                        self.antennas
                            .insert(antenna, vec![Pos(row as isize, col as isize)]);
                    }
                }
            }
        }
    }

    fn is_in_bounds(&self, pos: Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.row_n as isize && pos.1 < self.col_n as isize
    }

    fn get_antinodes(&self, pos1: &Pos, pos2: &Pos, max_dist: Option<usize>) -> Vec<Pos> {
        if pos1 == pos2 {
            return vec![];
        }
        let mut result: Vec<Pos> = vec![];
        let mut dist = 1;

        if let Some(max) = max_dist {
            if max > 1 {
                result.push((*pos1).clone());
                result.push((*pos2).clone());
            }
        } else {
            result.push((*pos1).clone());
            result.push((*pos2).clone());
        }

        loop {
            let row_diff = pos1.0 - pos2.0;
            let col_diff = pos1.1 - pos2.1;

            let new_pos = [
                Pos(pos1.0 + row_diff * dist, pos1.1 + col_diff * dist),
                Pos(pos2.0 - row_diff * dist, pos2.1 - col_diff * dist),
            ];

            let mut new_added = false;
            for &pos in &new_pos {
                if self.is_in_bounds(pos) {
                    result.push(pos.clone());
                    new_added = true;
                }
            }
            if !new_added {
                break;
            }

            dist += 1;
            if let Some(max_dist) = max_dist {
                if dist > max_dist as isize {
                    break;
                }
            }
        }
        result
    }

    fn count_antinodes(&mut self) -> usize {
        self.find_antennas();
        let mut result: HashSet<Pos> = HashSet::new();

        for &anntena_type in self.antennas.keys() {
            let locations = self.antennas.get(&anntena_type).unwrap();
            for i in 0..locations.len() {
                for j in i..locations.len() {
                    let locs = self.get_antinodes(&locations[i], &locations[j], self.max_dist);
                    for loc in locs {
                        result.insert(loc);
                    }
                }
            }
        }
        result.iter().count()
    }
}

fn main() {
    let mut map = Map::default();

    let mut idx = 0;
    loop {
        let input: String = read!("{}\n");
        if input.is_empty() {
            break;
        }
        let line: Vec<char> = input.chars().collect();
        idx += 1;
        map.grid.push(line);
    }

    map.row_n = idx;
    map.col_n = map.grid[0].len();
    map.max_dist = Some(1);
    println!("Locations with antinodes count: {}", map.count_antinodes());
    map.max_dist = None;
    println!(
        "Locations with antinodes (including a resonance) count: {}",
        map.count_antinodes()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_antinodes() {
        let mut map = Map::default();
        map.col_n = 10;
        map.row_n = 10;
        assert_eq!(
            map.get_antinodes(&Pos(7, 7), &Pos(8, 8), Some(1)),
            vec![Pos(6, 6), Pos(9, 9)]
        );
        assert_eq!(
            map.get_antinodes(&Pos(8, 8), &Pos(9, 9), Some(2)),
            vec![Pos(8, 8), Pos(9, 9), Pos(7, 7), Pos(6, 6)]
        );
        assert_eq!(map.get_antinodes(&Pos(8, 8), &Pos(8, 8), Some(1)), vec![]);
        assert_eq!(
            map.get_antinodes(&Pos(1, 0), &Pos(3, 0), Some(1)),
            vec![Pos(5, 0)]
        );
        assert_eq!(
            map.get_antinodes(&Pos(0, 0), &Pos(2, 0), None),
            vec![Pos(0, 0), Pos(2, 0), Pos(4, 0), Pos(6, 0), Pos(8, 0)]
        );
    }

    #[test]
    fn test_antinode_count_dist_1() {
        let mut map = Map::default();
        map.col_n = 12;
        map.row_n = 12;
        map.max_dist = Some(1);
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        map.grid = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(map.count_antinodes(), 14);
    }

    #[test]
    fn test_antinode_count_unlimited() {
        let mut map = Map::default();
        map.col_n = 12;
        map.row_n = 12;
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        map.grid = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        assert_eq!(map.count_antinodes(), 34);
    }
}
