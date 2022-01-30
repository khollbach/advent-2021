use std::collections::HashSet;
use std::io::BufRead;
use std::iter;
use itertools::Itertools;
use crate::solutions::day19::point::Point;
use crate::solutions::day19::scanner::Scanner;

pub fn read_input(input: impl BufRead) -> Vec<Scanner> {
    let mut lines = input.lines().map(Result::unwrap);

    let mut i = 0;
    iter::from_fn(|| {
        let s = read_scanner(&mut lines, i);
        i += 1;
        s
    }).collect()
}

/// Read the next scanner from the input stream. Returns None if `lines` is empty.
///
/// `i` is the expected index of this scanner.
fn read_scanner(lines: &mut impl Iterator<Item=String>, i: usize) -> Option<Scanner> {
    let header = lines.next()?;
    assert_eq!(header, format!("--- scanner {i} ---"));

    let mut beacons = HashSet::new();

    for line in lines {
        if line == "" { break; }

        if !beacons.insert(read_point(&line)) {
            panic!("Duplicate point in scanner {i}: {line}");
        }
    }

    Some(Scanner { beacons })
}

fn read_point(comma_separated: &str) -> Point {
    let (x, y, z) = comma_separated.split(',').map(|n| {
        n.parse().unwrap()
    }).collect_tuple().unwrap();

    Point::new(x, y, z)
}
