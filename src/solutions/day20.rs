use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> (Mask, Grid) {
    let mut lines = input.lines().map(Result::unwrap);

    let line = lines.next().unwrap();
    let mask: Vec<_> = line.chars().map(Pixel::from_char).collect();
    let mask: Mask = mask.try_into().expect("Wrong number of pixels in mask.");

    assert_eq!(lines.next().unwrap(), "");

    let grid = read_grid(lines);

    (mask, grid)
}

fn read_grid(lines: impl Iterator<Item=String>) -> Grid {
    let mut main_pixels = HashMap::new();

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            main_pixels.insert((i as i32, j as i32), Pixel::from_char(c));
        }
    }

    Grid {
        main_pixels,
        background_color: Pixel::Off,
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pixel {
    Off,
    On,
}

impl Pixel {
    fn from_char(c: char) -> Pixel {
        match c {
            '.' => Pixel::Off,
            '#' => Pixel::On,
            _ => panic!("Invalid pixel: {c}"),
        }
    }
}

type Mask = [Pixel; 2usize.pow(9)];

struct Grid {
    main_pixels: HashMap<Point, Pixel>,
    background_color: Pixel,
}

type Point = (i32, i32);

impl Grid {
    fn enhance(&mut self, mask: &Mask) {
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

const SAMPLE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

pub fn main() {
    let (mask, mut grid) = read_input(io::stdin().lock());
    // let (mask, mut grid) = read_input(SAMPLE.as_bytes()); // still works

    for _ in 0..2 {
        grid.enhance(&mask);
    }

    dbg!(grid.main_pixels.values().filter(|&&p| p == Pixel::On).count());
}
