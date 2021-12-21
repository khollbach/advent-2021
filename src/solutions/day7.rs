use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn read_input(input: impl BufRead) -> Vec<i32> {
    let (line,) = input.lines().map(Result::unwrap).collect_tuple().unwrap();

    line.split(',').map(|s| s.parse().unwrap()).collect()
}

pub fn main() {
    let positions = read_input(io::stdin().lock());

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let ans: i32 = (min..=max).map(|center| {
        positions.iter().map(|&p| (center - p).abs()).sum()
    }).min().unwrap();

    dbg!(ans);
}
