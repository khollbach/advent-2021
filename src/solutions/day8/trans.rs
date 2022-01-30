use std::collections::HashMap;
use crate::common::combinatorics::permutations;
use crate::solutions::day8::input::{Row, Signal};

pub struct Translation {
    mapping: HashMap<char, char>,
}

impl Translation {
    pub fn apply(&self, signal: &Signal) -> Signal {
        Signal::new(signal.segments_sorted.chars().map(|c| self.mapping[&c]))
    }

    /// By brute force, find a translation that maps every input signal to a real digit.
    pub fn decode(real_digits: &HashMap<Signal, usize>, row: &Row) -> Self {
        for trans in Translation::all() {
            if row.inputs.iter().all(|signal| {
                real_digits.contains_key(&trans.apply(signal))
            }) {
                return trans;
            }
        }

        panic!("Could not find a valid translation for row: {:?}", row);
    }

    /// Generate all 7! possible translations.
    fn all() -> impl Iterator<Item=Self> {
        let a_to_g: Vec<_> = ('a'..='g').collect();

        permutations(&a_to_g).into_iter().map(move |perm| {
            let inputs = perm.into_iter();
            let outputs = a_to_g.iter().cloned();

            Self {
                mapping: inputs.zip(outputs).collect()
            }
        })
    }
}
