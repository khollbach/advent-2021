use std::cmp::max;
use std::io;
use std::io::BufRead;
use itertools::Itertools;
use regex::Regex;

fn read_input(input: impl BufRead) -> Target {
    let (line,) = input.lines().map(Result::unwrap).collect_tuple().unwrap();

    // Note that the "minuses" in the y-coords are not optional.
    // Our approach assumes the target area is in the bottom-right quadrant.
    let regex = Regex::new(r"^target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)$").unwrap();
    let caps = regex.captures(&line).unwrap();

    let x_min = caps[1].parse().unwrap();
    let x_max = caps[2].parse().unwrap();
    assert!(x_min <= x_max);

    let y_min = caps[3].parse().unwrap();
    let y_max = caps[4].parse().unwrap();
    assert!(y_min <= y_max);

    Target { x_min, x_max, y_min, y_max }
}

pub fn main() {
    let target = read_input(io::stdin().lock());

    println!("{}", part_1(target));
    println!("{}", part_2(target));
}

/// Inclusive ranges.
///
/// y_min and y_max must be negative.
#[derive(Debug, Copy, Clone)]
struct Target {
    x_min: u32,
    x_max: u32,
    y_min: i32,
    y_max: i32,
}

impl Target {
    fn contains(self, x: u32, y: i32) -> bool {
        self.x_min <= x && x <= self.x_max &&
        self.y_min <= y && y <= self.y_max
    }
}

fn part_1(target: Target) -> i32 {
    sucessful_launches(target).max().unwrap()
}

fn part_2(target: Target) -> usize {
    sucessful_launches(target).count()
}

/// For each sucessful launch, return it's peak height.
fn sucessful_launches(target: Target) -> impl Iterator<Item=i32> {
    (0..=target.x_max).flat_map(move |dx| {
        (target.y_min ..= -target.y_min).filter_map(move |dy| {
            launch(dx, dy, target)
        })
    })
}

/// If we hit the target, return the peak height; else None.
fn launch(mut dx: u32, mut dy: i32, target: Target) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    loop {
        // Bang!
        if target.contains(x, y) {
            return Some(max_y);
        }

        // Ok we can stop trying now; it's all downhill from here.
        if y < target.y_min {
            return None;
        }

        // Take one "step".
        x += dx;
        y += dy;
        dx = dx.saturating_sub(1);
        dy -= 1;

        max_y = max(max_y, y);
    }
}
