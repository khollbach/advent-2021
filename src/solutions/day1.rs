use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<u32> {
    input.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

pub fn main() {
    let nums = read_input(io::stdin().lock());

    println!("{}", part_1(&nums));
}

fn part_1(nums: &[u32]) -> u32 {
    let mut count = 0;

    for i in 1..nums.len() {
        if nums[i-1] < nums[i] {
            count += 1;
        }
    }

    count
}
