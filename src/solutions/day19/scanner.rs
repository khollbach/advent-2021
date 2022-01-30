use std::collections::HashSet;
use crate::solutions::day19::point::Point;
use crate::solutions::day19::rotation::{all_rotations, Rotation};

#[derive(Debug)]
pub struct Scanner {
    /// Relative to this scanner's position and rotation.
    pub beacons: HashSet<Point>,
}

impl Scanner {
    /// Do these two scanners overlap? (They must share at least 12 beacons.)
    ///
    /// If so, return the rotated and translated version of `other` using
    /// `self`s frame of reference.
    ///
    /// Also returns the location of `other` in `self`s frame of reference.
    pub fn overlaps_with(&self, other: &Scanner) -> Option<(Scanner, Point)> {
        for other in other.all_rotations() {
            for &p1 in &self.beacons {
                for &p2 in &other.beacons {
                    // We assume p1 and p2 are the same point,
                    // then see if `self` and `other` overlap.
                    let delta = p1 - p2;
                    let other = other.translate(delta);

                    if self.beacons.intersection(&other.beacons).count() >= 12 {
                        return Some((other, delta));
                    }
                }
            }
        }

        None
    }

    /// All possible rotations of the current scanner.
    fn all_rotations<'a>(&'a self) -> impl Iterator<Item=Scanner> + 'a {
        all_rotations().map(|r| self.rotate(r))
    }

    fn rotate(&self, r: Rotation) -> Scanner {
        self.transform(|p| r.apply(p))
    }

    fn translate(&self, delta: Point) -> Scanner {
        self.transform(|p| p + delta)
    }

    /// Helper function for `rotate` and `translate`.
    fn transform(&self, f: impl Fn(Point) -> Point) -> Scanner {
        let beacons = self.beacons.iter().cloned().map(f).collect();
        Scanner { beacons }
    }
}
