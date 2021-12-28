use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use crate::common::input::read_digit_grid;

fn read_input(input: impl BufRead) -> HeightMap {
    let grid = read_digit_grid(input);
    HeightMap { grid }
}

struct HeightMap {
    grid: Vec<Vec<u32>>,
}

impl HeightMap {
    fn dims(&self) -> (usize, usize) {
        let r = self.grid.len();
        let c = self.grid[0].len();

        (r, c)
    }

    fn all_points(&self) -> impl Iterator<Item=(usize, usize)> {
        let (r, c) = self.dims();

        (0..r).flat_map(move |i| {
            (0..c).map(move |j| {
                (i, j)
            })
        })
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        self.all_points().filter(|&(i, j)| {
            self.neighbors(i, j).all(|(i2, j2)| {
                self.grid[i][j] < self.grid[i2][j2]
            })
        }).collect()
    }

    /// Sum of low-point risk levels.
    fn part_1(&self) -> u32 {
        self.low_points().into_iter().map(|(i, j)| {
            1 + self.grid[i][j]
        }).sum()
    }

    /// Product of 3 largest basin sizes.
    fn part_2(&self) -> u32 {
        let mut basin_sizes: Vec<_> = self.low_points().into_iter().map(|(i, j)| {
            let mut seen = HashSet::new();
            self.dfs(i, j, &mut seen);
            seen.len() as u32
        }).collect();

        basin_sizes.sort_unstable();
        basin_sizes[basin_sizes.len() - 3..].iter().product()
    }

    /// Visit everything reachable from (i, j), treating cells with the value 9 as walls.
    fn dfs(&self, i: usize, j: usize, seen: &mut HashSet<(usize, usize)>) {
        seen.insert((i, j));

        for (i2, j2) in self.neighbors(i, j) {
            if self.grid[i2][j2] != 9 && !seen.contains(&(i2, j2)) {
                self.dfs(i2, j2, seen);
            }
        }
    }

    /// 4-way directions: up/down/left/right.
    fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item=(usize, usize)> {
        let i = i as isize;
        let j = j as isize;
        let (r, c) = self.dims();
        let r = r as isize;
        let c = c as isize;

        [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().filter_map(move |(di, dj)| {
            let i2 = i + di;
            let j2 = j + dj;

            if 0 <= i2 && i2 < r &&
                0 <= j2 && j2 < c
            {
                Some((i2 as usize, j2 as usize))
            } else {
                None
            }
        })
    }
}

pub fn main() {
    let grid = read_input(io::stdin().lock());

    println!("{}", grid.part_1());
    println!("{}", grid.part_2());
}
