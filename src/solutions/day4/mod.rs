use std::collections::HashSet;
use std::io;
use crate::solutions::day4::input::read_input;
use crate::solutions::day4::cache::Cache;

mod input;
mod cache;

pub fn main() {
    let (draws, boards) = read_input(io::stdin().lock());

    println!("{}", part_1(&draws, boards.clone()));
    println!("{}", part_2(&draws, boards));
}

fn part_1(draws: &[u32], mut boards: Vec<Board>) -> u32 {
    let cache = Cache::new(&boards);
    let mut seen = HashSet::new();

    for &draw in draws {
        seen.insert(draw);

        for &hit in cache.hits.get(&draw).unwrap_or(&vec![]) {
            let board = &mut boards[hit.board_idx];

            if board.mark(hit.row, hit.col) {
                return board.sum_unmarked(&seen) * draw;
            }
        }
    }

    panic!("No winning board.");
}

fn part_2(draws: &[u32], mut boards: Vec<Board>) -> u32 {
    let cache = Cache::new(&boards);
    let mut seen = HashSet::new();

    let mut winners = vec![false; boards.len()];
    let mut num_winners = 0;

    for &draw in draws {
        seen.insert(draw);

        for &hit in cache.hits.get(&draw).unwrap_or(&vec![]) {
            let i = hit.board_idx;

            if boards[i].mark(hit.row, hit.col) && !winners[i]  {
                winners[i] = true;
                num_winners += 1;

                if num_winners == boards.len() {
                    return boards[i].sum_unmarked(&seen) * draw;
                }
            }
        }
    }

    panic!("Not all boards won. {} out of {}.", num_winners, boards.len());
}

#[derive(Debug, Clone)]
pub struct Board {
    grid: Vec<Vec<u32>>,
    row_hits: Vec<u32>,
    col_hits: Vec<u32>,
}

impl Board {
    /// Return true if there's a "bingo".
    fn mark(&mut self, row: usize, col: usize) -> bool {
        let r = &mut self.row_hits[row];
        let c = &mut self.col_hits[col];

        *r += 1;
        *c += 1;

        *r >= self.grid.len() as u32 || *c >= self.grid[0].len() as u32
    }

    fn all_values<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
        self.grid.iter().flat_map(|row| row.iter().copied())
    }

    fn sum_unmarked(&self, marked: &HashSet<u32>) -> u32 {
        self.all_values().filter(|x| !marked.contains(x)).sum()
    }
}
