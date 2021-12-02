use std::io;
use crate::solutions::day2::Dir::{Down, Forward, Up};
use crate::solutions::day2::input::read_input;

mod input;

pub enum Dir {
    Forward,
    Down,
    Up,
}

pub fn main() {
    let commands = read_input(io::stdin().lock());

    println!("{}", part_1(&commands));
}

fn part_1(commands: &[(Dir, u32)]) -> u32 {
    let mut depth = 0;
    let mut distance = 0;

    for (d, n) in commands {
        match d {
            Forward => {
                distance += n;
            }
            Down => {
                depth += n;
            }
            Up => {
                depth -= n;
            }
        }
    }

    depth * distance
}
