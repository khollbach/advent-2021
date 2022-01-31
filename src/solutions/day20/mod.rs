use std::io;
use crate::solutions::day20::input::read_input;

mod input;
mod grid;

pub fn main() {
    let (mask, mut grid) = read_input(io::stdin().lock());

    for _ in 0..2 {
        grid.enhance(&mask);
    }

    println!("{}", grid.lit_pixels().unwrap());
}
