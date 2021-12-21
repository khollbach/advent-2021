use std::collections::HashMap;
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

fn permutations<T: Copy>(elems: &[T]) -> Vec<Vec<T>> {
    if elems.is_empty() {
        // Just the empty permutation.
        return vec![vec![]];
    }

    let n = elems.len();
    let mut ret = Vec::with_capacity((1..=n).product()); // n!

    let x = elems[0];
    for sub_perm in permutations(&elems[1..]) {
        for i in 0..=sub_perm.len() {
            let mut perm = Vec::with_capacity(n);
            perm.extend(&sub_perm[..i]);
            perm.push(x);
            perm.extend(&sub_perm[i..]);

            ret.push(perm);
        }
    }

    ret
}
