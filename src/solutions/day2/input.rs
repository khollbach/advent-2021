use std::io::BufRead;
use itertools::Itertools;
use crate::solutions::day2::Dir;
use crate::solutions::day2::Dir::{Down, Forward, Up};

pub fn read_input(input: impl BufRead) -> Vec<(Dir, u32)> {
    input.lines().map(|line| {
        let (dir, num) = line.as_ref().unwrap().split(' ').collect_tuple().unwrap();

        (Dir::new(dir), num.parse().unwrap())
    }).collect()
}

impl Dir {
    fn new(s: &str) -> Self {
        match s {
            "forward" => Forward,
            "down" => Down,
            "up" => Up,
            _ => panic!("Invalid dir: {}", s),
        }
    }
}
