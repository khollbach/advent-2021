use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io;
use std::io::BufRead;
use crate::common::grid::neighbors_4_way;
use crate::common::input::read_digit_grid;

fn read_input(input: impl BufRead) -> Grid {
    let grid = read_digit_grid(input);
    Grid { grid }
}

pub fn main() {
    let grid = read_input(io::stdin().lock());

    println!("{}", grid.part_1());
    println!("{}", grid.part_2());
}

struct Grid {
    grid: Vec<Vec<u32>>,
}

type Point = (usize, usize);

impl Grid {
    fn part_1(&self) -> u32 {
        let start = (0, 0);
        let end = (self.grid.len() - 1, self.grid[0].len() - 1);

        self.shortest_path(start, end).unwrap()
    }

    fn part_2(&self) -> u32 {
        self.to_5x5().part_1()
    }

    /// Return the total cost of the least-cost path from start to end.
    fn shortest_path(&self, start: Point, end: Point) -> Option<u32> {
        let mut seen = HashSet::new();
        seen.insert(start);

        // Min-priority queue by distance from start.
        let mut to_visit = BinaryHeap::new();
        to_visit.push(Reverse((0, start)));

        while let Some(Reverse((d, p))) = to_visit.pop() {
            if p == end {
                return Some(d);
            }

            for p2 in self.neighbors(p) {
                // Note that each node's incoming edges all have the same cost, and we visit nodes
                // in increasing order of cost. Therefore, the first time we see a node, we know
                // we've taken the optimal path to it.
                //
                // Thus we never have to update the priority of a node we've already seen.
                if !seen.contains(&p2) {
                    seen.insert(p2);
                    to_visit.push(Reverse((d + self.get(p2), p2)));
                }
            }
        }

        None
    }

    fn get(&self, (i, j): Point) -> u32 {
        self.grid[i][j]
    }

    /// 4-way directions.
    fn neighbors(&self, (i, j): Point) -> impl Iterator<Item=Point> {
        neighbors_4_way(&self.grid, i, j)
    }

    /// Create a new grid (25x larger than self) based on the rules of the puzzle.
    fn to_5x5(&self) -> Self {
        let r = self.grid.len();
        let c = self.grid[0].len();

        let mut new_grid = Grid {
            grid: vec![vec![0; c * 5]; r * 5]
        };

        for i in 0..5 {
            for j in 0..5 {
                let increment = i + j;
                let offset = (r * i, c * j);
                new_grid.fill(self, increment as u32, offset);
            }
        }

        new_grid
    }

    /// Helper function for `to_5x5`.
    ///
    /// Fill a region of the current grid using `other`.
    /// The top-left corner of the region is given by the offset point.
    ///
    /// The entries are also increased by `increment`, and then wrapped (1-indexed)
    /// if they exceed 9.
    fn fill(&mut self, other: &Grid, increment: u32, (i_offset, j_offset): Point) {
        for i in 0..other.grid.len() {
            for j in 0..other.grid[0].len() {
                let x = other.grid[i][j] + increment;

                // Wrap x to be in the range 1..=9
                let x = (x - 1) % 9 + 1;

                self.grid[i_offset + i][j_offset + j] = x;
            }
        }
    }
}
