use std::collections::{HashSet, VecDeque};
use std::io;
use crate::solutions::day19::input::read_input;
use crate::solutions::day19::point::Point;
use crate::solutions::day19::scanner::Scanner;

mod input;
mod point;
mod scanner;
mod rotation;

pub fn main() {
    let scanners = read_input(io::stdin().lock());

    let (part1, part2) = solve(scanners);
    println!("{part1}");
    println!("{part2}");
}

fn solve(mut scanners: Vec<Scanner>) -> (usize, u32) {
    let locations = orient_scanners(&mut scanners);

    let beacons: HashSet<_> = scanners.iter().flat_map(|s| &s.beacons).collect();
    let num_beacons = beacons.len();

    // Maximum distance between any two points.
    let diameter = locations.iter().flat_map(|&p1| {
        locations.iter().map(move |&p2| {
            (p1 - p2).manhatten_norm()
        })
    }).max().unwrap();

    (num_beacons, diameter)
}

const ORIGIN: Point = Point::new(0, 0, 0);

/// Takes a while to run -- about 5 seconds on my laptop, built with --release.
///
/// Orient the scanners, so that each scanner's rotation and position is
/// relative to scanner 0.
///
/// Return the location of each scanner, relative to scanner 0's orientation.
fn orient_scanners(scanners: &mut Vec<Scanner>) -> Vec<Point> {
    let n = scanners.len();
    let mut locations = vec![ORIGIN; n];

    let mut unreached: HashSet<_> = (0..n).collect();
    let mut q = VecDeque::with_capacity(n);

    locations[0] = ORIGIN;
    unreached.remove(&0);
    q.push_back(0);

    // BFS, trying to reach every scanner.
    while let Some(i) = q.pop_front() {
        for j in unreached.clone() {
            if let Some((new_j, location)) = scanners[i].overlaps_with(&scanners[j]) {
                // Update the orientation of scanner j to be relative to scanner 0.
                scanners[j] = new_j;

                locations[j] = location;
                unreached.remove(&j);
                q.push_back(j);
            }
        }
    }

    assert!(unreached.is_empty(), "The scanner graph wasn't connected. unreached: {unreached:?}");

    locations
}
