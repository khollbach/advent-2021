use std::io::BufRead;
use itertools::Itertools;
use crate::solutions::day5::Point;

pub fn read_input(input: impl BufRead) -> Vec<(Point, Point)> {
    input.lines().map(|line| {
        line.as_ref().unwrap().split(" -> ").map(Point::new).collect_tuple().unwrap()
    }).collect()
}

impl Point {
    fn new(comma_separated: &str) -> Self {
        let (x, y) = comma_separated.split(',').map(|s| {
            s.parse().unwrap()
        }).collect_tuple().unwrap();

        Self { x, y }
    }
}
