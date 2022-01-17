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

    println!("{}", part_1(trees.iter().cloned()));
    println!("{}", part_2(&trees));
}

fn part_1(trees: impl Iterator<Item=Tree>) -> u32 {
    let sum = trees.reduce(Tree::add).unwrap();
    sum.magnitude()
}

fn part_2(trees: &[Tree]) -> u32 {
    let n = trees.len();

    (0..n).flat_map(|i| {
        (0..n).filter_map(move |j| {
            if i == j {
                None
            } else {
                let sum = Tree::add(trees[i].clone(), trees[j].clone());
                Some(sum.magnitude())
            }
        })
    }).max().unwrap()
}
