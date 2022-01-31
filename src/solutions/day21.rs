use std::{io, iter};
use crate::solutions::day21::dice::DetD100;
use crate::solutions::day21::input::read_input;

mod input;
mod dice;

pub fn main() {
    let (pos1, pos2) = read_input(io::stdin().lock());

    println!("{}", play_game(pos1, pos2));
}

/// Returns the loser's score times the number of die rolls.
fn play_game(pos1: u32, pos2: u32) -> u32 {
    let mut die = DetD100::new();
    let mut players = [
        Player::new(pos1),
        Player::new(pos2),
    ];

    for i in iter::repeat([0, 1]).flatten() {
        players[i].move_(die.roll_3_times());

        if players[i].score >= 1000 {
            return players[1 - i].score * die.num_rolls();
        }
    }

    unreachable!();
}

struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(start_pos: u32) -> Self {
        Self {
            pos: start_pos,
            score: 0,
        }
    }

    fn move_(&mut self, amount: u32) {
        self.pos += amount;
        wrap_1_idx(&mut self.pos, 10);

        self.score += self.pos;
    }
}

/// Wrap `x` to be in the range `1..=n`.
fn wrap_1_idx(x: &mut u32, n: u32) {
    assert_ne!(n, 0);

    if *x == 0 {
        *x = n;
    } else {
        *x -= 1;
        *x %= n;
        *x += 1;
    }
}
