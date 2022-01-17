use std::fmt;
use std::fmt::{Display, Formatter};
use crate::solutions::day18::tree::Tree;
use crate::solutions::day18::tree::Tree::{Leaf, Pair};

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Leaf { value } => write!(f, "{}", value),
            Pair { left, right } => write!(f, "[{},{}]", left, right),
        }
    }
}
