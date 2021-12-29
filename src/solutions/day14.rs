use std::collections::HashMap;
use std::{io, mem};
use std::io::BufRead;
use itertools::Itertools;

type Rules = HashMap<(char, char), char>;

fn read_input(input: impl BufRead) -> (String, Rules) {
    let mut lines = input.lines().map(Result::unwrap);

    let polymer = lines.next().unwrap();

    assert_eq!(lines.next().unwrap(), "");

    let mut rules = HashMap::new();

    for line in lines {
        let (ab, c) = line.split(" -> ").collect_tuple().unwrap();
        let (a, b) = ab.chars().collect_tuple().unwrap();
        let (c,) = c.chars().collect_tuple().unwrap();

        if let Some(old_c) = rules.insert((a, b), c) {
            panic!("Repeated key: {} -- old/new values: {} {}", ab, old_c, c);
        }
    }

    (polymer, rules)
}

pub fn main() {
    let (polymer, rules) = read_input(io::stdin().lock());

    println!("{}", part_1(&polymer, &rules));
}

fn part_1(polymer: &str, rules: &Rules) -> u32 {
    assert!(!polymer.is_empty());

    // Initial frequencies of character pairs.
    let mut freqs = HashMap::new();
    for (a, b) in polymer.chars().tuple_windows() {
        *freqs.entry((a, b)).or_default() += 1;
    }

    for _ in 0..10 {
        step(&mut freqs, rules);
    }

    // Count the first character in each pair.
    let mut individual_freqs = HashMap::<_, u32>::new();
    for ((a, _), f) in freqs {
        *individual_freqs.entry(a).or_default() += f;
    }

    // Edge case: the last char in the string isn't counted above.
    let last_c = polymer.chars().next_back().unwrap();
    *individual_freqs.entry(last_c).or_default() += 1;

    // We want the largest individual freq minus the smallest one.
    let mut fs = individual_freqs.into_iter().map(|(_, f)| f).sorted_unstable();
    let min_f = fs.next().unwrap();
    let max_f = fs.next_back().unwrap();

    max_f - min_f
}

/// Update frequencies of each pair according to the rules.
fn step(freqs: &mut HashMap<(char, char), u32>, rules: &Rules) {
    let mut new_freqs = HashMap::new();

    for ((a, b), f) in mem::take(freqs) {
        let mid = rules[&(a, b)];

        *new_freqs.entry((a, mid)).or_default() += f;
        *new_freqs.entry((mid, b)).or_default() += f;
    }

    *freqs = new_freqs;
}
