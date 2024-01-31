use std::collections::HashSet;

use crate::models::SudokuValue;

pub struct Guess {
    pub index: usize,
    pub value: u8,
    pub other_posibilities: HashSet<u8>,
    pub state: Vec<SudokuValue>,
}

impl Guess {
    pub fn other_guess(&self) -> Option<Guess> {
        match self.other_posibilities.iter().next() {
            Some(x) => {
                let mut new_possibilities = self.other_posibilities.clone();
                new_possibilities.remove(&x);
                Some(Guess {
                    index: self.index,
                    value: *x,
                    other_posibilities: new_possibilities,
                    state: self.state.clone(),
                })
            }
            None => None,
        }
    }
}
