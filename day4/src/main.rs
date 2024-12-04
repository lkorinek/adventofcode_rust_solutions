use std::io;

enum Task { First, Second }
const DIR_MOVES: [(i32, i32); 8] = [(1, 0), (1, 1), (1, -1), (-1, 0),
                                    (-1, 1), (-1, -1), (0, 1), (0, -1)];
const SIZE: usize = 140;
const WINNING_SEQUENCE: [char; 4] = ['X', 'M', 'A', 'S'];

struct WordSearch {
    puzzle: [[char; SIZE]; SIZE],
    word_count: u64,
}

impl WordSearch {
    fn new() -> Self {
        Self {
            puzzle: [['N'; SIZE]; SIZE],
            word_count: 0,
        }
    }

    fn fill_board(&mut self) {
        for row in 0..SIZE {
            let mut input = String::new();
            io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
             for (col, piece_str) in input.trim().chars().enumerate() {
                self.puzzle[row][col] = piece_str;
            }
        }
    }

    fn is_in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < SIZE as i32 && col >= 0 && col < SIZE as i32
    }

    fn get_new_coords(&self, row: usize, col: usize, dir: (i32, i32)) -> Option<(usize, usize)> {
        let coords = (row as i32 + dir.0, col as i32 + dir.1);
        if self.is_in_bounds(coords.0, coords.1) {
            return Some((coords.0 as usize, coords.1 as usize))
        }
        None
    }

    fn run(&mut self, task: Task) {
        self.word_count = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                self.word_count += match task {
                    Task::First => self.search_dirs_task1(row, col),
                    Task::Second => self.search_dirs_task2(row, col),
                }
            }
        }
    }

    fn get_word_count(&self) -> u64 {
        self.word_count
    }

    fn is_char_in_sequence(&self, letter: char, idx: usize) -> bool {
        letter == WINNING_SEQUENCE[idx]
    }

    fn search_dirs_task1(&self, row: usize, col: usize) -> u64 {
        let mut word_count = 0;
        for dir in DIR_MOVES {
            let mut curr_row = row;
            let mut curr_col = col;
            let mut idx_count = 0;

            while self.is_char_in_sequence(self.puzzle[curr_row][curr_col], idx_count) {

                idx_count += 1;
                if idx_count == WINNING_SEQUENCE.len() {
                    word_count += 1;
                    break
                }

                if let Some((r, c)) = self.get_new_coords(curr_row, curr_col, dir) {
                    curr_row = r;
                    curr_col = c;
                } else {
                    break;
                }
            }
        }
        word_count
    }

    fn is_letter_at_position(&self, row: i32, col: i32, letter: char) -> bool {
        self.is_in_bounds(row, col) && self.puzzle[row as usize][col as usize] == letter
    }

    fn search_dirs_task2(&self, row: usize, col: usize) -> u64 {
        let row = row as i32;
        let col = col as i32;
        if self.is_letter_at_position(row, col, 'A')
            && (self.is_letter_at_position(row+1, col+1, 'M') && self.is_letter_at_position(row-1, col-1, 'S') ||
                self.is_letter_at_position(row+1, col+1, 'S') && self.is_letter_at_position(row-1, col-1, 'M'))
            && (self.is_letter_at_position(row-1, col+1, 'M') && self.is_letter_at_position(row+1, col-1, 'S') ||
                self.is_letter_at_position(row-1, col+1, 'S') && self.is_letter_at_position(row+1, col-1, 'M')) {
            return 1
        }
        0
    }
}

fn main() {
    let mut board = WordSearch::new();
    board.fill_board();
    board.run(Task::First);
    println!("Number of words found in the puzzle 1: {}", board.get_word_count());
    board.run(Task::Second);
    println!("Number of words found in the puzzle 2: {}", board.get_word_count());
}
