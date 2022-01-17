use std::str::FromStr;
use crate::solutions::day18::Tree;
use crate::solutions::day18::tree::parse::StackItem::{Comma, Node, Open};
use crate::solutions::day18::tree::Tree::Leaf;

/// Helper enum for `Tree::from_str`.
#[derive(Debug, Eq, PartialEq)]
enum StackItem {
    Open,
    Comma,
    Node(Tree),
}

impl FromStr for Tree {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                '[' => stack.push(Open),
                ',' => stack.push(Comma),
                ']' => {
                    let right = stack.pop().unwrap();
                    let comma = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let open = stack.pop().unwrap();

                    assert_eq!(open, Open);
                    assert_eq!(comma, Comma);

                    let pair = match (left, right) {
                        (Node(left), Node(right)) => Self::pair(left, right),
                        (a, b) => panic!("Failed to parse: [a,b] where a and b were {:?} and {:?}", a, b),
                    };
                    stack.push(Node(pair));
                }
                _ => {
                    let value = c.to_digit(10).unwrap();
                    stack.push(Node(Leaf { value }));
                }
            }
        }

        assert_eq!(stack.len(), 1, "Failed to parse; stack: {:?}", stack);

        match stack.pop().unwrap() {
            Node(tree) => Ok(tree),
            item => panic!("Failed to parse; item: {:?}", item),
        }
    }
}
