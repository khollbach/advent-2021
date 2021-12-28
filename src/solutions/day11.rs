use std::io;
use std::io::BufRead;
use crate::common::grid;
use crate::common::input::read_digit_grid;

fn read_input(input: impl BufRead) -> Octopi {
    let grid = read_digit_grid(input);
    Octopi { grid }
}

pub fn main() {
    let mut octopi = read_input(io::stdin().lock());

    println!("{}", octopi.simulate(100));
}

struct Octopi {
    grid: Vec<Vec<u32>>,
}

impl Octopi {
    /// Simulate many steps, and return the total number of flashes.
    fn simulate(&mut self, num_steps: usize) -> u32 {
        let mut total = 0;

        for _ in 0..num_steps {
            total += self.step();
        }

        total
    }

    /// Simulate one step, and return the number of flashes.
    fn step(&mut self) -> u32 {
        let mut to_flash = vec![];

        // Initial increment.
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                self.increment(i, j, &mut to_flash);
            }
        }

        let mut num_flashes = 0;

        // Trigger flashes (these may set off more flashes).
        while let Some((i, j)) = to_flash.pop() {
            // Flash!
            num_flashes += 1;

            for (i2, j2) in self.neighbors(i, j) {
                self.increment(i2, j2, &mut to_flash);
            }
        }

        // Reset octopi that flashed during this round.
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                let energy_level = &mut self.grid[i][j];
                if *energy_level >= 10 {
                    *energy_level = 0;
                }
            }
        }

        num_flashes
    }

    /// Helper function for `step`.
    ///
    /// Increase an energy level by 1.
    ///
    /// When an energy level hits 10 for the first time, push it to `to_flash`.
    fn increment(&mut self, i: usize, j: usize, to_flash: &mut Vec<(usize, usize)>) {
        let energy_level = &mut self.grid[i][j];
        *energy_level += 1;

        if *energy_level == 10 {
            to_flash.push((i, j));
        }
    }

    /// 8-way directions, including diagonals.
    fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item=(usize, usize)> {
        grid::neighbors_8_way(&self.grid, i, j)
    }
}
