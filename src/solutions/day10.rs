use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<String> {
    input.lines().map(Result::unwrap).collect()
}

pub fn main() {
    let lines = read_input(io::stdin().lock());

    println!("{}", part_1(&lines));
}

/// Return the total score of all corrupted lines.
fn part_1(lines: &[String]) -> u32 {
    lines.iter().filter_map(|line| {
        detect_corruption(&line)
    }).sum()
}

/// Return the corruption score of this line, if any.
fn detect_corruption(line: &str) -> Option<u32> {
    // Track open symbols that haven't yet "found their match".
    let mut stack = vec![];

    for c in line.chars() {
        if "([{<".contains(c) { // open
            stack.push(c);
        } else {
            assert!(")]}>".contains(c)); // close
            let open = stack.pop();

            // Is c the right kind of closing symbol?
            if open != Some(close_to_open(c)) {
                return Some(corruption_score(c));
            }
        }
    }

    None
}

fn close_to_open(closing_symbol: char) -> char {
    match closing_symbol {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Not an closing symbol: {}", closing_symbol),
    }
}

fn corruption_score(closing_symbol: char) -> u32 {
    match closing_symbol {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Not a closing symbol: {}", closing_symbol),
    }
}
