use std::io::BufRead;
use itertools::Itertools;
use regex::Regex;
use crate::solutions::day13::input::FoldType::{X, Y};

#[derive(Debug)]
pub struct Input {
    pub points: Vec<(usize, usize)>,
    pub folds: Vec<(FoldType, usize)>,
}

#[derive(Debug, Copy, Clone)]
pub enum FoldType {
    X,
    Y,
}

pub fn read_input(input: impl BufRead) -> Input {
    let mut lines = input.lines().map(Result::unwrap);

    let mut points = vec![];
    for line in &mut lines {
        if line == "" {
            break;
        }

        let (x, y) = line.split(',').map(|n| {
            n.parse().unwrap()
        }).collect_tuple().unwrap();

        points.push((x, y));
    }

    let regex = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();

    let mut folds = vec![];
    for line in lines {
        let caps = regex.captures(&line).unwrap();
        let fold_type = match &caps[1] {
            "x" => X,
            "y" => Y,
            _ => unreachable!(),
        };
        let idx = caps[2].parse().unwrap();

        folds.push((fold_type, idx));
    }

    Input { points, folds }
}
