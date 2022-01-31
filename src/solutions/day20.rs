use std::collections::HashSet;
use std::io;
use std::io::BufRead;

fn read_input(input: impl BufRead) -> (Mask, Grid) {
    let mut lines = input.lines().map(Result::unwrap);

    let line = lines.next().unwrap();
    let mask: Vec<_> = line.chars().map(char_to_pixel).collect();
    let mask: Mask = mask.try_into().expect("Wrong number of pixels in mask.");

    assert_eq!(lines.next().unwrap(), "");

    let mut lit_pixels = HashSet::new();
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if char_to_pixel(c) {
                lit_pixels.insert((i as i32, j as i32));
            }
        }
    }
    let grid = Grid { lit_pixels };

    (mask, grid)
}

fn char_to_pixel(c: char) -> bool {
    match c {
        '.' => false,
        '#' => true,
        _ => panic!("Invalid pixel {c}"),
    }
}

type Mask = [bool; 2usize.pow(9)];

struct Grid {
    lit_pixels: HashSet<Point>,
}

type Point = (i32, i32);

impl Grid {
    fn enhance(&mut self, mask: &Mask) {
        let relevant_points: HashSet<_> = self.lit_pixels.iter().flat_map(|&p| {
            neighborhood(p)
        }).collect();

        self.lit_pixels = relevant_points.into_iter().filter(|&p| {
            let idx = self.get_neighborhood(p);
            mask[idx]
        }).collect();
    }

    /// Get the surrounding pixels, and interpret them as a binary number.
    fn get_neighborhood(&self, p: Point) -> usize {
        let mut answer = 0;
        for p2 in neighborhood(p) {
            answer <<= 1;
            answer |= self.lit_pixels.contains(&p2) as usize;
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
    // let (mask, mut grid) = read_input(SAMPLE.as_bytes()); // works

    for _ in 0..2 {
        grid.enhance(&mask);
    }

    dbg!(grid.lit_pixels.len());
}
