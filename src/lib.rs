mod common;
mod solutions;

pub fn solve(day: u32) {
    assert!(1 <= day && day <= 25);

    use solutions::*;
    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        6 => day6::main(),
        7 => day7::main(),
        8 => day8::main(),
        9 => day9::main(),
        10 => day10::main(),
        11 => day11::main(),
        12 => day12::main(),
        13 => day13::main(),
        _ => panic!("Not yet implemented: Day {}", day),
    }
}
