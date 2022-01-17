use std::io;
use std::io::BufRead;
use crate::solutions::day18::tree::Tree;

mod tree;

fn read_input(input: impl BufRead) -> Vec<Tree> {
    input.lines().map(|line| {
        line.unwrap().parse().unwrap()
    }).collect()
}

pub fn main() {
    let trees = read_input(io::stdin().lock());

    println!("{}", part_1(trees.into_iter()));
}

fn part_1(trees: impl Iterator<Item=Tree>) -> u32 {
    let sum = trees.reduce(Tree::add).unwrap();
    sum.magnitude()
}
