use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn read_input(input: impl BufRead) -> Vec<usize> {
    let (line,) = input.lines().map(Result::unwrap).collect_tuple().unwrap();

    line.split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug, Clone)]
struct FishTimers {
    /// `timers[t]` is the number of fish with an internal timer of t.
    timers: [u64; 9],
}

impl FishTimers {
    fn new(individual_fish: impl Iterator<Item=usize>) -> Self {
        let mut timers = [0; 9];

        for t in individual_fish {
            timers[t] += 1;
        }

        Self { timers }
    }

    /// Run the simulation for `num_days` and return the total number of fish.
    fn simulate(mut self, num_days: u32) -> u64 {
        for _ in 0..num_days {
            self.evolve();
        }

        self.count()
    }

    /// Simulate one day of time passing.
    fn evolve(&mut self) {
        // Each fish's internal timer goes down by one.
        // Timers wrap around 0 back up to 8.
        self.timers.rotate_left(1);

        // Each timer that wrapped spawns a new fish with timer 6.
        self.timers[6] += self.timers[8];
    }

    /// The total number of fish.
    fn count(&self) -> u64 {
        self.timers.iter().copied().sum()
    }
}

pub fn main() {
    let input = read_input(io::stdin().lock());
    let fish = FishTimers::new(input.into_iter());

    println!("{}", fish.clone().simulate(80));
    println!("{}", fish.simulate(256));
}
