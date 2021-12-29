use std::fmt::{Display, Formatter};
use std::{fmt, io};
use crate::solutions::day13::input::FoldType::{X, Y};
use crate::solutions::day13::input::{FoldType, read_input};

mod input;

pub fn main() {
    let input = read_input(io::stdin().lock());
    let grid = Grid::new(&input.points);

    let (fold_type, idx) = input.folds[0];
    println!("{}", part_1(grid.clone(), fold_type, idx));

    part_2(grid, &input.folds);
}

fn part_1(mut grid: Grid, fold_type: FoldType, idx: usize) -> usize {
    grid.fold(fold_type, idx);
    grid.num_points()
}

fn part_2(mut grid: Grid, folds: &[(FoldType, usize)]) {
    for &(fold_type, idx) in folds {
        grid.fold(fold_type, idx);
    }

    println!("{}", grid);
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(points: &[(usize, usize)]) -> Self {
        assert!(!points.is_empty());

        let max_x = points.iter().map(|&(x, _)| x).max().unwrap();
        let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

        let num_rows = max_y + 1;
        let num_cols = max_x + 1;

        let mut grid = vec![vec![false; num_cols]; num_rows];

        for &(x, y) in points {
            grid[y][x] = true;
        }

        Self { grid }
    }

    fn num_points(&self) -> usize {
        self.grid.iter().flat_map(|row| {
            row.iter().filter(|&&b| b)
        }).count()
    }

    fn fold(&mut self, fold_type: FoldType, idx: usize) {
        match fold_type {
            X => self.fold_x(idx),
            Y => self.fold_y(idx),
        }
    }

    /// Fold to the left along the vertical line x=`x`.
    fn fold_x(&mut self, x: usize) {
        let num_rows = self.grid.len();
        let num_cols = self.grid[0].len();
        assert!(x >= num_cols / 2);

        // Fold each row "in half".
        for y in 0..num_rows {
            assert!(!self.grid[y][x], "Folded along a point.");

            let mut i = 1;
            while x + i < num_cols {
                self.grid[y][x - i] |= self.grid[y][x + i];
                i += 1;
            }

            // Shrink to fit, throwing away the folded-over half.
            self.grid[y].truncate(x);
        }
    }

    /// Fold up along the horizontal line y=`y`.
    fn fold_y(&mut self, y: usize) {
        let num_rows = self.grid.len();
        let num_cols = self.grid[0].len();
        assert!(y >= num_rows / 2);

        // Fold each column "in half".
        for x in 0..num_cols {
            assert!(!self.grid[y][x], "Folded along a point.");

            let mut i = 1;
            while y + i < self.grid.len() {
                self.grid[y - i][x] |= self.grid[y + i][x];
                i += 1;
            }
        }

        // Shrink to fit, throwing away the folded-up half.
        self.grid.truncate(y);
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in &self.grid {
            let line: String = row.iter().map(|&b| match b {
                false => ' ',
                true => '*',
            }).collect();

            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}
