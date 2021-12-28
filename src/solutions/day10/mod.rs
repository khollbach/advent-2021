use std::io;
use std::io::BufRead;
use crate::solutions::day10::ParseResult::{Corrupt, Incomplete};
use crate::solutions::day10::symbol::{Open, Symbol};

mod symbol;

fn read_input(input: impl BufRead) -> Vec<String> {
    input.lines().map(Result::unwrap).collect()
}

pub fn main() {
    let lines = read_input(io::stdin().lock());

    println!("{}", part_1(&lines));
    println!("{}", part_2(&lines));
}

/// Return the total score of all corrupted lines.
fn part_1(lines: &[String]) -> u32 {
    lines.iter().filter_map(|line| match parse(&line) {
        Corrupt(score) => Some(score),
        _ => None,
    }).sum()
}

/// Return the median score of all incomplete lines.
fn part_2(lines: &[String]) -> u64 {
    let mut scores: Vec<_> = lines.iter().filter_map(|line| match parse(&line) {
        Incomplete(score) => Some(score),
        _ => None,
    }).collect();

    // We're promised an odd number.
    assert_eq!(scores.len() % 2, 1);

    // Median.
    scores.sort_unstable();
    scores[scores.len() / 2]
}

enum ParseResult {
    Ok,
    Corrupt(u32),
    Incomplete(u64),
}

/// Process this line and detect if it's corrupt or incomplete.
fn parse(line: &str) -> ParseResult {
    // Track open symbols that haven't yet "found their match".
    let mut stack = vec![];

    for sym in line.chars().map(Symbol::new) {
        match sym {
            Symbol::Open(open) => stack.push(open),
            Symbol::Close(close) => {
                let open = stack.pop();

                // Do we have the right kind of closing symbol?
                if open.map(Open::to_close) != Some(close) {
                    return Corrupt(close.corruption_score());
                }
            }
        }
    }

    // Any leftovers?
    if stack.is_empty() {
        ParseResult::Ok
    } else {
        Incomplete(completion_score(stack))
    }
}

fn completion_score(mut stack: Vec<Open>) -> u64 {
    let mut score = 0;

    while let Some(open) = stack.pop() {
        score *= 5;
        score += open.to_close().completion_score();
    }

    score
}
