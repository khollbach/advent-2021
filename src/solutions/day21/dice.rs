use std::iter;

/// A die that rolls 1, 2, ..., 99, 100, 1, 2, ..., 99, 100, etc.
pub struct DetD100 {
    /// An infinite sequence of upcoming rolls.
    future_rolls: Box<dyn Iterator<Item=u32>>,
    /// How many rolls so far?
    num_rolls: u32,
}

impl DetD100 {
    pub fn new() -> Self {
        Self {
            future_rolls: Box::new(iter::repeat(1..=100).flatten()),
            num_rolls: 0,
        }
    }

    /// Returns the sum of the rolls.
    pub fn roll_3_times(&mut self) -> u32 {
        self.num_rolls += 3;
        (&mut self.future_rolls).take(3).sum()
    }

    pub fn num_rolls(&self) -> u32 {
        self.num_rolls
    }
}
