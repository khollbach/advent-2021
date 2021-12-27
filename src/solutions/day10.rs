use std::io;
use std::io::BufRead;
use crate::solutions::day10::ParseResult::{Corrupt, Incomplete};

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

    for c in line.chars() {
        if "([{<".contains(c) { // open
            stack.push(c);
        } else {
            assert!(")]}>".contains(c)); // close
            let open = stack.pop();

            // Is c the right kind of closing symbol?
            if open.map(open_to_close) != Some(c) {
                return Corrupt(corruption_score(c));
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

fn open_to_close(open: char) -> char {
    match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Not an opening symbol: {}", open),
    }
}

fn corruption_score(close: char) -> u32 {
    match close {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Not a closing symbol: {}", close),
    }
}

fn completion_score(mut stack: Vec<char>) -> u64 {
    let mut score = 0;

    while let Some(open) = stack.pop() {
        let close = open_to_close(open);

        score *= 5;
        score += char_completion_score(close);
    }

    score
}

fn char_completion_score(close: char) -> u64 {
    match close {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Not a closing symbol: {}", close),
    }
}
