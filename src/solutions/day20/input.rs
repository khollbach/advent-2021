use std::collections::HashMap;
use std::io::BufRead;
use crate::solutions::day20::grid::{Grid, Mask, Pixel};

pub fn read_input(input: impl BufRead) -> (Mask, Grid) {
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

    Grid::new(main_pixels)
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
