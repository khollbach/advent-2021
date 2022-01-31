use std::io::BufRead;
use itertools::Itertools;
use regex::Regex;

pub fn read_input(input: impl BufRead) -> (u32, u32) {
    let regex = Regex::new(r"^Player (\d+) starting position: (\d+)$").unwrap();

    input.lines().zip(1..).map(|(line, idx)| {
        let caps = regex.captures(line.as_ref().unwrap()).unwrap();

        let (player, pos) = caps.iter().skip(1).map(|n| {
            n.unwrap().as_str().parse().unwrap()
        }).collect_tuple().unwrap();

        assert_eq!(player, idx);

        pos
    }).collect_tuple().unwrap()
}
