use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Pixel {
    Off,
    On,
}

pub type Mask = [Pixel; 2usize.pow(9)];

#[derive(Clone)]
pub struct Grid {
    main_pixels: HashMap<Point, Pixel>,
    background_color: Pixel,
}

type Point = (i32, i32);

impl Grid {
    pub fn new(main_pixels: HashMap<Point, Pixel>) -> Grid {
        Grid {
            main_pixels,
            background_color: Pixel::Off,
        }
    }

    pub fn enhance(&mut self, mask: &Mask) {
        let relevant_points: HashSet<_> = self.main_pixels.keys().flat_map(|&p| {
            neighborhood(p)
        }).collect();

        self.main_pixels = relevant_points.into_iter().map(|p| {
            let idx = self.get_neighborhood(p);
            (p, mask[idx])
        }).collect();

        self.background_color = match self.background_color {
            Pixel::Off => mask[0],
            Pixel::On => *mask.last().unwrap(),
        };
    }

    /// Get the surrounding pixels, and interpret them as a binary number.
    fn get_neighborhood(&self, p: Point) -> usize {
        let mut answer = 0;
        for p2 in neighborhood(p) {
            answer <<= 1;
            let pixel = *self.main_pixels.get(&p2).unwrap_or(&self.background_color);
            answer |= pixel as usize;
        }
        answer
    }

    /// Return None if infinitely many pixels are lit.
    pub fn lit_pixels(&self) -> Option<usize> {
        match self.background_color {
            Pixel::Off => {
                let n = self.main_pixels.values().filter(|&&p| p == Pixel::On).count();
                Some(n)
            }
            Pixel::On => None,
        }
    }
}

/// Return the 9 points in the 3x3 square centered at `(i, j)`.
///
/// Row-major order.
fn neighborhood((i, j): Point) -> impl Iterator<Item=Point> {
    (-1..=1).flat_map(move |di| {
        (-1..=1).map(move |dj| {
            (i + di, j + dj)
        })
    })
}
