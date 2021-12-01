use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<u32> {
    input.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

pub fn main() {
    let nums = read_input(io::stdin().lock());

    println!("{}", part_1(&nums));
    println!("{}", part_2(&nums));
}

fn part_1(nums: &[u32]) -> usize {
    nums.windows(2).filter(|pair| pair[0] < pair[1]).count()
}

fn part_2(nums: &[u32]) -> usize {
    let window_sums: Vec<_> = nums.windows(3).map(|triple| triple.iter().sum()).collect();
    part_1(&window_sums)
}
