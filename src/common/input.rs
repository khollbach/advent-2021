use std::io::BufRead;

pub fn read_digit_grid(input: impl BufRead) -> Vec<Vec<u32>> {
    input.lines().map(|line| {
        line.unwrap().chars().map(|c| {
            c.to_digit(10).unwrap()
        }).collect()
    }).collect()
}
