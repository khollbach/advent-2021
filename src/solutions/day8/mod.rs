use std::collections::HashMap;
use std::io;
use crate::solutions::day8::input::{read_input, Row, Signal};
use crate::solutions::day8::trans::Translation;

mod input;
mod trans;

pub fn main() {
    let rows = read_input(io::stdin().lock());

    println!("{}", part_1(&rows));
    println!("{}", part_2(&rows));
}

/// The total number of "easy" digits in all the output digits.
fn part_1(rows: &[Row]) -> usize {
    let unique_lengths = [2, 3, 4, 7];

    rows.iter().flat_map(|r| {
        r.outputs.iter().filter(|d| {
            unique_lengths.contains(&d.segments_sorted.len())
        })
    }).count()
}

const REAL_SIGNALS: [&str; 10] = [
    "abcefg",
    "cf",
    "acdeg",
    "acdfg",
    "bcdf",

    "abdfg",
    "abdefg",
    "acf",
    "abcdefg",
    "abcdfg",
];

/// For each row, figure out the translation for that row.
///
/// Interpret each row's output digits as a 4-digit number, and take the sum over all rows.
fn part_2(rows: &[Row]) -> u32 {
    let real_digits: HashMap<_, _> = REAL_SIGNALS.into_iter().enumerate().map(|(i, s)| (Signal::from_str(s), i)).collect();

    rows.iter().map(|r| {
        let trans = Translation::decode(&real_digits, r);

        // Translate and interpret each output digit, and concat them into a string.
        let answer: String = r.outputs.iter().map(|d| real_digits[&trans.apply(&d)].to_string()).collect();

        answer.parse::<u32>().unwrap()
    }).sum()
}
