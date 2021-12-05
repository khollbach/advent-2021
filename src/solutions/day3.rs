use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<String> {
    input.lines().map(Result::unwrap).collect()
}

pub fn main() {
    let lines = read_input(io::stdin().lock());

    let (gamma, epsilon) = part_1(&lines);
    println!("{}", gamma * epsilon);

    let (o2, co2) = part_2(&lines);
    println!("{}", o2 * co2);
}

fn part_1(lines: &[String]) -> (u32, u32) {
    let word_len = lines[0].len();
    let mut num_ones = vec![0; word_len];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                num_ones[i] += 1;
            } else {
                assert_eq!(c, '0');
            }
        }
    }

    let gamma: String = num_ones.into_iter().map(|n| {
        if n >= lines.len() / 2 {
            '1'
        } else {
            '0'
        }
    }).collect();
    let gamma = u32::from_str_radix(&gamma, 2).unwrap();

    // Flip each bit of gamma.
    let mask = (1 << word_len) - 1;
    let epsilon = gamma ^ mask;

    (gamma, epsilon)
}

fn part_2(lines: &[String]) -> (u32, u32) {
    let o2 = filter_down(lines, true);
    let co2 = filter_down(lines, false);

    (o2, co2)
}

/// Partition on the 1st bit, then the 2nd bit, ... etc, until there's only one choice left.
fn filter_down(lines: &[String], majority: bool) -> u32 {
    let mut candidates: Vec<&str> = lines.iter().map(String::as_str).collect();

    let mut i = 0;
    while candidates.len() > 1 {
        let (maj, min) = partition(candidates.into_iter(), i);
        candidates = if majority { maj } else { min };

        i += 1;
    }

    u32::from_str_radix(&candidates[0], 2).unwrap()
}

/// Partition on the ith bit. Return (majority, minority).
fn partition<'a>(lines: impl Iterator<Item=&'a str>, i: usize) -> (Vec<&'a str>, Vec<&'a str>) {
    let (zeros, ones) = lines.partition::<Vec<_>, _>(|line| &line[i..i + 1] == "0");

    // Break ties in favour of ones being majority.
    if ones.len() >= zeros.len() {
        (ones, zeros)
    } else {
        (zeros, ones)
    }
}
