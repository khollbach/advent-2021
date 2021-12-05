use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<String> {
    input.lines().map(Result::unwrap).collect()
}

pub fn main() {
    let lines = read_input(io::stdin().lock());

    let (gamma, epsilon) = part_1(&lines);
    println!("{}", gamma * epsilon);
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
