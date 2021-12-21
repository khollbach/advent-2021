use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn read_input(input: impl BufRead) -> Vec<Row> {
    input.lines().map(|line| {
        let (inputs, outputs) = line.as_ref().unwrap().split(" | ").collect_tuple().unwrap();

        Row {
            signal_patterns: inputs.split(' ').map(String::from).collect(),
            output_digits: outputs.split(' ').map(String::from).collect(),
        }
    }).collect()
}

struct Row {
    /// Guaranteed to be 10 unique signal patterns.
    signal_patterns: Vec<String>,

    /// 4 signal patterns that we want to decode.
    output_digits: Vec<String>,
}

pub fn main() {
    let rows = read_input(io::stdin().lock());

    println!("{}", part_1(&rows));
}

/// The total number of "easy" digits in all the output digits.
fn part_1(rows: &[Row]) -> usize {
    let unique_lengths = [2, 3, 4, 7];

    rows.iter().flat_map(|r| {
        r.output_digits.iter().filter(|d| {
            unique_lengths.contains(&d.len())
        })
    }).count()
}
