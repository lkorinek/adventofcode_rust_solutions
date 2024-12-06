use std::collections::HashSet;
use std::fmt;
use text_io::read;
use day6::{Guard, Dir, MapObject, Pos};

const DATA_LENGTH: usize = 130;
struct Board {
    guard: Guard,
    guard_backup: Guard,
    rows: usize,
    cols: usize,
    map: [Vec<MapObject>; DATA_LENGTH],
    map_backup: [Vec<MapObject>; DATA_LENGTH],
    count: u32,
    path: HashSet<Pos>,
}

impl Board {
    fn new() -> Self {
        Self {
            guard: Guard {pos: Pos(0,0), dir: Dir::Up},
            guard_backup: Guard {pos: Pos(0,0), dir: Dir::Up},
            rows: DATA_LENGTH,
            cols: 0,
            map: std::array::from_fn(|_| Vec::new()),
            map_backup: std::array::from_fn(|_| Vec::new()),
            count: 1,
            path: HashSet::new(),
        }
    }

    fn backup_map(&mut self) {
        self.guard_backup = self.guard.clone();
        self.map_backup = self.map.clone();
    }

    fn restore_map(&mut self) {
        self.guard = self.guard_backup.clone();
        self.map = self.map_backup.clone();
        self.count = 0;
    }

    fn is_in_bounds(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 < self.rows as isize && pos.1 < self.cols as isize
    }

    fn try_move(&mut self, pos: &Pos) -> bool {
        if self.is_in_bounds(&pos) && self.map[pos.0 as usize][pos.1 as usize] != MapObject::Obstacle {
            if self.map[self.guard.pos.0 as usize][self.guard.pos.1 as usize] != MapObject::Visited {
                self.count += 1;
            }
            self.map[self.guard.pos.0 as usize][self.guard.pos.1 as usize] = MapObject::Visited;
            self.guard.pos = pos.clone();
            self.path.insert(self.guard.pos);
            return true
        }
        false
    }

    fn move_guard(&mut self) -> bool {

        let mut count = 0;
        while count < 4 {
            let new_pos = &self.guard.pos + self.guard.dir.get_change();
            if !self.is_in_bounds(&new_pos) {
                return false
            }
            if self.try_move(&(&self.guard.pos + self.guard.dir.get_change())) {
                return true
            }
            self.guard.rotate();
            count += 1;
        }
        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str_result = String::new();
        for row in &self.map {
            for cell in row {
                str_result.push_str(&*cell.to_string());
            }
            str_result.push_str(&*"\n".to_owned());
        }
        write!(f, "{str_result}")
    }
}

fn main() {
    let mut board = Board::new();

    for row in 0..DATA_LENGTH {
        let input: String = read!("{}\n");
        let parts = input.chars();
        board.cols = input.chars().count();

        for (col, part) in parts.enumerate() {
            board.map[row].push(
                match part {
                    '#' => MapObject::Obstacle,
                    '^' => {
                        board.guard.pos = Pos(row as isize, col as isize);
                        MapObject::Guard
                    },
                    _ => MapObject::Empty,
                }
            );
        }
    }
    board.backup_map();
    while board.move_guard() {}
    println!("Guard move count: {}", board.count);
    let original_path = board.path.clone();

    let mut loop_count = 0;
    for pos in original_path {
        let (row, col) = (pos.0, pos.1);

        if row == board.guard_backup.pos.0 && col == board.guard_backup.pos.1 {
            continue
        }

        board.restore_map();
        let mut is_loop = HashSet::new();

        board.map[row as usize][col as usize] = MapObject::Obstacle;
        while board.move_guard() {
            if !is_loop.insert(board.guard.clone()) {
                loop_count += 1;
                break
            }
        }
    }
    println!("Guard stuck in a loop count: {}", loop_count);
}
