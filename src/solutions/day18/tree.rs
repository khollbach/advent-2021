use std::borrow::{Borrow, BorrowMut};
use std::cmp::max;
use crate::solutions::day18::tree::Tree::{Leaf, Pair};

mod parse;
mod print;

/// A node in a "full" tree.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Tree {
    Pair {
        left: Box<Tree>,
        right: Box<Tree>,
    },
    Leaf {
        value: u32,
    },
}

impl Tree {
    pub fn add(left: Self, right: Self) -> Self {
        assert!(left.height() <= 4);
        assert!(right.height() <= 4);

        let mut sum = Self::pair(left, right);
        sum.reduce();
        sum
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            Leaf { value } => *value,
            Pair { left, right } => {
                3 * left.magnitude() + 2 * right.magnitude()
            }
        }
    }

    fn height(&self) -> usize {
        match self {
            Leaf { .. } => 0,
            Pair { left, right } => {
                1 + max(left.height(), right.height())
            }
        }
    }

    fn pair(left: Self, right: Self) -> Self {
        let left = Box::new(left);
        let right = Box::new(right);

        Pair { left, right }
    }

    fn reduce(&mut self) {
        assert!(self.height() <= 5);

        while self.try_explode() || self.try_split() {}
    }

    /// Helper function for `reduce`.
    fn try_explode(&mut self) -> bool {
        if let Some(DeepPair { prev_val, pair, next_val }) = self.first_deep_pair() {
            let (left, right) = pair.unwrap_simple_pair();
            *pair = Leaf { value: 0 };

            if let Some(val) = prev_val {
                *val += left;
            }
            if let Some(val) = next_val {
                *val += right;
            }

            true
        } else {
            false
        }
    }

    /// Helper function for `try_explode`.
    ///
    /// Find the first "deep" pair. A pair is deep if it's nested inside 4 other pairs.
    ///
    /// We also return the leaf values that come before and after that pair.
    fn first_deep_pair<'a>(&'a mut self) -> Option<DeepPair<'a>> {
        let mut prev_val = None;
        let mut deep_pair = None;
        let mut next_val = None;

        let mut stack = vec![(0, self)];

        // In-order traversal.
        while let Some((depth, node)) = stack.pop() {
            // Grab the first deep pair.
            if deep_pair.is_none() && node.is_pair() && depth == 4 {
                deep_pair = Some(node);
                continue;
            }

            match node {
                Pair { left, right } => {
                    // Left gets popped before right.
                    stack.push((depth + 1, right.borrow_mut()));
                    stack.push((depth + 1, left.borrow_mut()));
                }
                Leaf { value } => if deep_pair.is_none() {
                    prev_val = Some(value);
                } else {
                    next_val = Some(value);
                    break;
                }
            }
        }

        deep_pair.map(|pair| {
            DeepPair { prev_val, pair, next_val }
        })
    }

    fn is_pair(&self) -> bool {
        match self {
            Pair { .. } => true,
            Leaf { .. } => false,
        }
    }

    /// Panics if self is not a simple pair.
    fn unwrap_simple_pair(&self) -> (u32, u32) {
        match self {
            Pair { left, right } => match (left.borrow(), right.borrow()) {
                (Leaf { value: left_val }, Leaf { value: right_val }) => (*left_val, *right_val),
                _ => panic!("Not a simple pair."),
            }
            _ => panic!("Not a pair."),
        }
    }

    /// Helper function for `reduce`.
    fn try_split(&mut self) -> bool {
        if let Some(node) = self.first_big_leaf() {
            let value = match node {
                Leaf { value } => *value,
                _ => unreachable!(),
            };

            let left = Box::new(Leaf {
                value: value / 2,
            });
            let right = Box::new(Leaf {
                value: value / 2 + value % 2,
            });
            *node = Pair { left, right };

            true
        } else {
            false
        }
    }

    /// Helper function for `try_split`.
    ///
    /// In-order traversal; return the first "big" leaf: value 10 or greater.
    fn first_big_leaf(&mut self) -> Option<&mut Tree> {
        match self {
            Leaf { value } => if *value >= 10 {
                Some(self)
            } else {
                None
            }
            Pair { left, right } => {
                left.first_big_leaf().or_else(|| right.first_big_leaf())
            }
        }
    }
}

/// Return type of `Tree::first_deep_pair`.
struct DeepPair<'a> {
    prev_val: Option<&'a mut u32>,
    pair: &'a mut Tree,
    next_val: Option<&'a mut u32>,
}
