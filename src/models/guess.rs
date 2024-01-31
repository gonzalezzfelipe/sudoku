use std::collections::HashSet;

use crate::models::SudokuCell;

#[derive(Debug)]
pub struct Guess {
    pub index: usize,
    pub value: u8,
    pub other_possibilities: HashSet<u8>,
    pub state: Vec<SudokuCell>,
}

impl Guess {
    /// Create new guess from other possilities.
    pub fn other_guess(&self) -> Option<Guess> {
        match self.other_possibilities.iter().next() {
            Some(x) => {
                let mut new_possibilities = self.other_possibilities.clone();
                new_possibilities.remove(&x);
                Some(Guess {
                    index: self.index,
                    value: *x,
                    other_possibilities: new_possibilities,
                    state: self.state.clone(),
                })
            }
            None => None,
        }
    }
}
