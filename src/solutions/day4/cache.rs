use std::collections::HashMap;
use crate::solutions::day4::Board;

/// Not really a cache, but idk what else to call this.
pub struct Cache {
    /// Map each number to where it appears in the boards.
    pub hits: HashMap<u32, Vec<Hit>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub board_idx: usize,
    pub row: usize,
    pub col: usize,
}

impl Cache {
    pub fn new(boards: &[Board]) -> Self {
        let mut ret = Self { hits: HashMap::new() };
        for (i, b) in boards.iter().enumerate() {
            ret.insert(i, b);
        }
        ret
    }

    fn insert(&mut self, board_idx: usize, board: &Board) {
        for (row, vals) in board.grid.iter().enumerate() {
            for (col, &val) in vals.iter().enumerate() {
                self.hits.entry(val).or_default().push(Hit { board_idx, row, col });
            }
        }
    }
}
