use lazy_static::lazy_static;
use crate::common::combinatorics::permutations;
use crate::solutions::day19::point::Point;

/// An axis-aligned rotation of 3-D space.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rotation {
    /// A linear transformation represented as a matrix.
    matrix: Matrix,
}

impl Rotation {
    pub fn apply(self, p: Point) -> Point {
        self.matrix.apply(p)
    }
}

/// A 3x3 matrix with integer values.
///
/// Corresponds to a particular linear transformation of 3-D space.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Matrix {
    rows: [Point; 3],
}

impl Matrix {
    pub fn apply(self, p: Point) -> Point {
        let coords = self.rows.map(|row| {
            row.dot_product(p)
        });
        Point { coords }
    }
}

/// Generate all possible `Rotation`s. There should be 24 of them.
///
/// Cached, since the result never changes.
pub fn all_rotations() -> impl Iterator<Item=Rotation> {
    lazy_static! {
        static ref ANSWER: Vec<Rotation> = all_rotations_helper();
    }

    // Sanity checks.
    assert_eq!(ANSWER.len(), 24);
    debug_assert!(ANSWER.contains(&Rotation { matrix: Matrix { rows: IDENTITY }}));

    ANSWER.iter().copied()
}

const X_AXIS: Point = Point::new(1, 0, 0);
const Y_AXIS: Point = Point::new(0, 1, 0);
const Z_AXIS: Point = Point::new(0, 0, 1);

const IDENTITY: [Point; 3] = [
    X_AXIS,
    Y_AXIS,
    Z_AXIS,
];

/// Generate all possible `Rotation`s.
fn all_rotations_helper() -> Vec<Rotation> {
    // Row-permutations of the 3x3 identity matrix.
    let perms = permutations(&IDENTITY).into_iter().map(|perm| {
        // Covert `perm` from a Vec to an array, so that it's properly a matrix.
        let rows: [Point; 3] = perm.try_into().unwrap();
        Matrix { rows }
    });

    // Subsets of {0, 1, 2}.
    let idx_subsets = powerset(&[0, 1, 2]);

    let rots_and_refls = perms.flat_map(|perm| {
        idx_subsets.iter().map(move |negated_axes| {
            // Negate a subset of the rows of `perm`.
            let mut matrix = perm.clone();
            for &i in negated_axes {
                matrix.rows[i] *= -1;
            }
            matrix
        })
    });

    let rotations = rots_and_refls.filter_map(|matrix| {
        // Filter out axis-aligned transformations that involve _reflection_.
        // (Should be exactly half of them.)
        if is_valid_rotation(matrix) {
            Some(Rotation { matrix })
        } else {
            None
        }
    });

    rotations.collect()
}

/// Helper function for `all_rotations_helper`.
///
/// `matrix` should be a permutation of the identity matrix, with some rows negated.
fn is_valid_rotation(matrix: Matrix) -> bool {
    // Get the column vectors of `matrix`.
    let [x, y, z] = IDENTITY.map(|axis| {
        matrix.apply(axis)
    });

    // This should hold only for rotations.
    // Reflections would send z in the negative direction.
    x.cross_product(y) == z
}

/// Helper function for `all_rotations_helper`.
///
/// Generate all (not-necessarily contiguous) subsequences of `elems`.
/// That is, for each element in the input, either include it or don't.
///
/// Think of this as computing all subsets of the input.
fn powerset<T: Copy>(elems: &[T]) -> Vec<Vec<T>> {
    if elems.is_empty() {
        // Just the empty set.
        return vec![vec![]];
    }

    let n = elems.len();
    let x = elems[n-1]; // Last element.

    let without_x = powerset(&elems[..n-1]);

    let mut with_x = without_x.clone();
    for set in &mut with_x {
        set.push(x);
    }

    let mut all = without_x;
    all.extend(with_x);
    all
}
