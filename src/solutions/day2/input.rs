use std::io::BufRead;
use crate::solutions::day2::Dir;
use crate::solutions::day2::Dir::{Down, Forward, Up};

pub fn read_input(input: impl BufRead) -> Vec<(Dir, u32)> {
    input.lines().map(|line| {
        let line = line.unwrap();

        let mut words = line.split(' ');
        let dir = Dir::new(words.next().unwrap());
        let num = words.next().unwrap().parse().unwrap();
        assert!(words.next().is_none());

        (dir, num)
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
