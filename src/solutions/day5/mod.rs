use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::io;
use std::ops::AddAssign;
use crate::solutions::day5::input::read_input;

mod input;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

pub fn main() {
    let pairs = read_input(io::stdin().lock());

    println!("{}", solve(&pairs, false));
    println!("{}", solve(&pairs, true));
}

fn solve(pairs: &[(Point, Point)], part_2: bool) -> usize {
    let mut freqs = HashMap::<_, u32>::new();

    for &(p1, p2) in pairs {
        // In part 1, only consider horizontal and vertical lines.
        if part_2 || p1.x == p2.x || p1.y == p2.y {
            let incr = compute_increment(p1, p2);

            let mut p = p1;
            loop {
                *freqs.entry(p).or_default() += 1;

                if p == p2 { break; }

                p += incr;
            }
        }
    }

    freqs.values().filter(|&&c| c >= 2).count()
}

fn compute_increment(p1: Point, p2: Point) -> Point {
    let x = increment_1d(p1.x, p2.x);
    let y = increment_1d(p1.y, p2.y);

    Point { x, y }
}

fn increment_1d(x1: i32, x2: i32) -> i32 {
    match x1.cmp(&x2) {
        Less => 1,
        Equal => 0,
        Greater => -1,
    }
}
