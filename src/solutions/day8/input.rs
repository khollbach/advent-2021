use std::io::BufRead;
use itertools::Itertools;

pub fn read_input(input: impl BufRead) -> Vec<Row> {
    input.lines().map(|line| {
        let (inputs, outputs) = line.as_ref().unwrap().split(" | ").collect_tuple().unwrap();

        // We don't really do any validation here, just fyi.
        Row {
            inputs: inputs.split(' ').map(Signal::from_str).collect(),
            outputs: outputs.split(' ').map(Signal::from_str).collect(),
        }
    }).collect()
}

#[derive(Debug)]
pub struct Row {
    /// Guaranteed to be 10 unique signal patterns.
    pub inputs: Vec<Signal>,

    /// 4 signal patterns that we want to decode.
    pub outputs: Vec<Signal>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Signal {
    pub segments_sorted: String,
}

impl Signal {
    pub fn from_str(segments: &str) -> Self {
        Self::new(segments.chars())
    }

    pub fn new(segments: impl Iterator<Item=char>) -> Self {
        let mut chars: Vec<_> = segments.collect();
        chars.sort_unstable();

        let segments_sorted: String = chars.into_iter().collect();
        Self { segments_sorted }
    }
}
