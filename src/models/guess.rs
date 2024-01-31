use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

use crate::models::SudokuCell;

#[derive(Eq, PartialEq, Clone)]
pub struct Guess {
    pub index: usize,
    pub value: u8,
    pub other_possibilities: HashSet<u8>,
    pub state: Vec<SudokuCell>,
}

impl Hash for Guess {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.value.hash(state);
        self.state.hash(state);
        let mut sorted_vec: Vec<u8> = self.other_possibilities.clone().into_iter().collect();
        sorted_vec.sort();
        for value in sorted_vec {
            value.hash(state);
        }
    }
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(
            f,
            "Guess {{ index: {}, value: {} }})",
            self.index, self.value
        )
    }
}
impl fmt::Debug for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(
            f,
            "Guess {{ index: {}, value: {} }})",
            self.index, self.value
        )
    }
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
