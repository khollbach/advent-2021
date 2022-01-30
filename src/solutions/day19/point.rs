use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub coords: [i32; 3],
}

impl Point {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            coords: [x, y, z],
        }
    }

    pub fn dot_product(self, other: Point) -> i32 {
        let mut sum = 0;
        for i in 0..self.coords.len() {
            sum += self.coords[i] * other.coords[i];
        }
        sum
    }

    pub fn cross_product(self, other: Point) -> Point {
        let [a1, a2, a3] = self.coords;
        let [b1, b2, b3] = other.coords;

        // Taken from wikipedia:
        // https://en.wikipedia.org/wiki/Cross_product#Coordinate_notation
        let c1 = a2 * b3 - a3 * b2;
        let c2 = a3 * b1 - a1 * b3;
        let c3 = a1 * b2 - a2 * b1;

        Point::new(c1, c2, c3)
    }

    pub fn manhatten_norm(self) -> u32 {
        self.coords.into_iter().map(|x| {
            x.abs() as u32
        }).sum()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(mut self, other: Point) -> Point {
        for i in 0..self.coords.len() {
            self.coords[i] += other.coords[i];
        }
        self
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(mut self, other: Point) -> Point {
        for i in 0..self.coords.len() {
            self.coords[i] -= other.coords[i];
        }
        self
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, scalar: i32) -> Point {
        let coords = self.coords.map(|x| x * scalar);
        Point { coords }
    }
}

impl MulAssign<i32> for Point {
    fn mul_assign(&mut self, scalar: i32) {
        *self = *self * scalar;
    }
}
