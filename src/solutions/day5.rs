use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> Vec<(Point, Point)> {
    input.lines().map(|line| {
        let mut words = line.as_ref().unwrap().split(" -> ");
        let p1 = words.next().unwrap();
        let p2 = words.next().unwrap();
        assert!(words.next().is_none());

        (Point::new(p1), Point::new(p2))
    }).collect()
}

impl Point {
    fn new(comma_separated: &str) -> Self {
        let mut coords = comma_separated.split(',');
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        assert!(coords.next().is_none());

        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let pairs = read_input(io::stdin().lock());

    let mut freqs = HashMap::<_, u32>::new();

    for (p1, p2) in pairs {
        if p1.x == p2.x {
            let x = p1.x;
            let y_min = min(p1.y, p2.y);
            let y_max = max(p1.y, p2.y);
            for y in y_min..=y_max {
                *freqs.entry(Point { x, y }).or_default() += 1;
            }
        } else if p1.y == p2.y {
            let y = p1.y;
            let x_min = min(p1.x, p2.x);
            let x_max = max(p1.x, p2.x);
            for x in x_min..=x_max {
                *freqs.entry(Point { x, y }).or_default() += 1;
            }
        }
    }

    let num_overlaps = freqs.values().filter(|&&c| c >= 2).count();
    println!("{}", num_overlaps);
}
