use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> HeightMap {
    let grid = input.lines().map(|line| {
        line.unwrap().chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect()
    }).collect();

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

    /// Sum of low-point heights.
    fn part_1(&self) -> u32 {
        self.low_points().into_iter().map(|(i, j)| {
            1 + self.grid[i][j]
        }).sum()
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
}
