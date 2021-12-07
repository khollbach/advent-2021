use std::io::BufRead;
use crate::solutions::day4::Board;

pub fn read_input(input: impl BufRead) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines().map(Result::unwrap).peekable();

    let draws = lines.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    assert!(lines.next().unwrap().is_empty());

    let mut boards = vec![];
    while lines.peek().is_some() {
        boards.push(Board::new(&mut lines));
    }

    (draws, boards)
}

impl Board {
    /// Read up to and including the first blank line, or EOF.
    fn new(lines: &mut impl Iterator<Item=String>) -> Self {
        let mut grid = vec![];

        for line in lines {
            if line.trim().is_empty() {
                break;
            }

            let row: Vec<_> = line.split_whitespace().map(|word| word.parse().unwrap()).collect();
            grid.push(row);
        }

        // Non-empty rectangle.
        assert!(!grid.is_empty());
        assert!(!grid[0].is_empty());
        assert!(grid.iter().all(|row| row.len() == grid[0].len()));

        Self {
            row_hits: vec![0; grid.len()],
            col_hits: vec![0; grid[0].len()],
            grid,
        }
    }
}
