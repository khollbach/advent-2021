use std::io;
use crate::solutions::day20::grid::{Grid, Mask};
use crate::solutions::day20::input::read_input;

mod input;
mod grid;

pub fn main() {
    let (mask, grid) = read_input(io::stdin().lock());

    println!("{}", simulate(grid.clone(), &mask, 2));

    // A little slow; takes almost 1 second on my laptop, built with --release.
    println!("{}", simulate(grid, &mask, 50));
}

/// Returns the number of lit pixels at the end.
fn simulate(mut grid: Grid, mask: &Mask, num_steps: usize) -> usize {
    for _ in 0..num_steps {
        grid.enhance(mask);
    }

    grid.lit_pixels().unwrap()
}
