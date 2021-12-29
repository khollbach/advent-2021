use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn read_input(input: impl BufRead) -> Vec<i32> {
    let (line,) = input.lines().map(Result::unwrap).collect_tuple().unwrap();

    line.split(',').map(|s| s.parse().unwrap()).collect()
}

pub fn main() {
    let positions = read_input(io::stdin().lock());

    println!("{}", min_fuel(&positions, |x| x));
    println!("{}", min_fuel(&positions, triangular_number));
}

/// `cost_fn(d)` is how much fuel it costs to move distance d >= 0.
///
/// We want to find a `center` position that minimizes the total fuel spent
/// for all crabs to move to that position.
fn min_fuel(positions: &[i32], cost_fn: impl Fn(u32) -> u32) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    // For each possible center point:
    (min..=max).map(|center| {
        // Add up all the fuel costs.
        positions.iter().map(|&p| {
            let dist = (center - p).abs() as u32;
            cost_fn(dist)
        }).sum()
    }).min().unwrap()
}

/// 1 + 2 + ... + n
fn triangular_number(n: u32) -> u32 {
    n * (n + 1) / 2
}
