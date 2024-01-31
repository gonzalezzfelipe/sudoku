use std::collections::{HashMap, HashSet, VecDeque};

use crate::errors::{SudokuCreationError, UnsolvableSudokuError};
use crate::models::{Guess, SudokuValue};
use crate::utils::{assert_values, print_sudoku, GroupIndexes};

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub values: Vec<SudokuValue>,
}

impl From<&str> for Sudoku {
    fn from(s: &str) -> Sudoku {
        let v: Vec<&str> = s.split(",").collect();

        assert!(v.len() == 81);

        let values: Vec<u8> = v
            .into_iter()
            .map(|x| match x.trim().parse::<u8>() {
                Ok(parsed) => parsed,
                Err(_) => 0,
            })
            .collect();

        Sudoku::new(values).expect("Invalid")
    }
}

impl Sudoku {
    pub fn new(values: Vec<u8>) -> Result<Sudoku, SudokuCreationError> {
        match assert_values(values.clone()) {
            Ok(()) => {}
            Err(err) => return Err(err),
        }

        // Init posibilities
        let mut parsed_values: Vec<SudokuValue> = vec![];

        for &value in values.iter() {
            parsed_values.push(SudokuValue::new(value));
        }
        Ok(Sudoku {
            values: parsed_values,
        })
    }

    pub fn print(&self) {
        print_sudoku(self.clone());
    }

    pub fn is_solved(&self) -> bool {
        let full_hash_set = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        for indexes in GroupIndexes::new() {
            let mut hash_set = HashSet::new();
            for index in indexes {
                if let Some(value) = self.values[index].value {
                    hash_set.insert(value);
                } else {
                    return false;
                }
            }
            if full_hash_set != hash_set {
                return false;
            }
        }
        true
    }

    pub fn filter(&mut self) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = false;
        for indexes in GroupIndexes::new() {
            let mut values_in_indexes: HashSet<u8> = HashSet::new();

            for &index in &indexes {
                if let Some(value) = self.values[index].value {
                    values_in_indexes.insert(value);
                }
            }

            for &index in &indexes {
                if self.values[index].value == None {
                    let mut posibilities =
                        self.values[index].posibilities.as_ref().unwrap().clone();
                    for item in values_in_indexes.iter() {
                        modified |= posibilities.remove(item);
                    }

                    if posibilities.is_empty() {
                        return Err(UnsolvableSudokuError);
                    }
                    self.values[index].posibilities = Some(posibilities);
                }
            }
        }
        Ok(modified)
    }

    fn check(&mut self) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = false;
        for indexes in GroupIndexes::new() {
            let mut values_in_indexes_counter: HashMap<u8, i32> = HashMap::new();

            // Populate counter of posibilities in the indexes. i.e: Count how many times
            // a 7 appears in a row, or column, or block.
            for &index in &indexes {
                if self.values[index].value == None {
                    if let Some(posibilities) = self.values[index].posibilities.clone() {
                        for &posibility in posibilities.iter() {
                            let count = values_in_indexes_counter.entry(posibility).or_insert(0);
                            *count += 1;
                        }
                    } else {
                        return Err(UnsolvableSudokuError);
                    }
                }
            }

            for &index in &indexes {
                if self.values[index].value == None {
                    // If there is a index with a posibility that only occurs once, then
                    // we want to set that value. ie: You can only put a 7 in a specific
                    // place in a row.
                    for &posibility in self.values[index].posibilities.clone().unwrap().iter() {
                        if let Some(&count) = &values_in_indexes_counter.get(&posibility) {
                            if count == 1 {
                                self.values[index].posibilities = Some(HashSet::from([posibility]));
                                modified = true;
                            }
                        }
                    }
                }
            }
        }
        Ok(modified)
    }

    fn clean_posibilities(&mut self) {
        for index in 0..81 {
            if self.values[index].value == None {
                let posibilities = self.values[index].posibilities.as_ref().unwrap().clone();
                if posibilities.len() == 1 {
                    for item in posibilities.iter() {
                        self.values[index].set_value(*item)
                    }
                }
            }
        }
    }

    fn apply_guess(&mut self, guess: &Guess) {
        self.values = guess.state.clone();
        self.values[guess.index] = SudokuValue {
            value: Some(guess.value),
            posibilities: None,
            is_original_value: false,
        };
    }

    fn reverse_guess(&mut self, guess: &Guess) {
        self.values = guess.state.clone();
    }

    fn guess(&mut self, guesses: &mut VecDeque<Guess>) {
        // Choose place to start guessing.
        let mut index = 0;
        while self.values[index].value != None {
            index += 1;
        }
        let value_to_guess = self.values[index].clone();

        // Copy posibilities, and extract one.
        let mut guess_posibilities = value_to_guess.posibilities.unwrap().clone();
        let value = guess_posibilities.iter().next().unwrap().clone();
        guess_posibilities.remove(&value);
        let guess = Guess {
            index,
            value,
            other_posibilities: guess_posibilities,
            state: self.values.clone(),
        };

        // Replace value in sudoku values, and retry.
        self.apply_guess(&guess);
        guesses.push_back(guess);
    }

    fn run_routine(&mut self) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = true;

        while modified {
            modified = false;
            modified = match self.filter() {
                Ok(value) => modified || value,
                Err(err) => return Err(err),
            };
            modified = match self.check() {
                Ok(value) => modified || value,
                Err(err) => return Err(err),
            };

            self.clean_posibilities();
        }

        Ok(modified)
    }

    /// Try and solve the sudoku without guessing.
    ///
    /// Generally speaking, there are hard and easy sudokus. Easy sudokus can be solved
    /// without guessing. Hard sudokus are the ones that cannot be solved without guessing
    /// and checking that the sudoku is solvable.
    pub fn try_solve(&mut self) -> Result<(), UnsolvableSudokuError> {
        let mut modified = true;
        while modified {
            modified = match self.run_routine() {
                Ok(value) => value,
                Err(err) => return Err(err),
            };
        }
        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), UnsolvableSudokuError> {
        let mut is_solved = false;
        let mut guesses: VecDeque<Guess> = VecDeque::new();
        let mut iterations = 0;

        while !is_solved && iterations < 999 {
            match self.try_solve() {
                Ok(()) => {
                    is_solved = self.is_solved();
                    if is_solved {
                        return Ok(());
                    }
                    self.guess(&mut guesses);
                }
                Err(_) => match guesses.pop_back() {
                    Some(guess) => match guess.other_guess() {
                        Some(new_guess) => {
                            self.apply_guess(&new_guess);
                            guesses.push_back(new_guess);
                        }
                        None => {
                            self.reverse_guess(&guess);
                            self.guess(&mut guesses);
                        }
                    },
                    None => {
                        return Err(UnsolvableSudokuError);
                    }
                },
            }
            iterations += 1;
        }
        Err(UnsolvableSudokuError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        #[rustfmt::skip]
        let values: Vec<u8> = vec![
            0, 0, 0, 0, 0, 2, 7, 0, 5,
            0, 0, 7, 0, 1, 0, 0, 0, 0,
            5, 8, 0, 0, 0, 0, 4, 2, 0,
            0, 3, 5, 0, 9, 8, 0, 0, 0,
            7, 1, 0, 0, 2, 0, 0, 0, 9,
            0, 0, 0, 0, 0, 5, 0, 4, 0,
            1, 6, 8, 2, 0, 9, 5, 0, 3,
            0, 0, 0, 3, 0, 1, 0, 8, 0,
            0, 5, 2, 7, 0, 6, 9, 1, 4,
        ];
        Sudoku::new(values).unwrap();
    }

    #[test]
    fn test_filter_row_complete() {
        // Test first row.
        let mut values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        for _ in 0..72 {
            values.push(0);
        }

        let mut sudoku = Sudoku::new(values).unwrap();
        let _ = sudoku.filter();

        assert_eq!(sudoku.values[8].posibilities, Some(HashSet::from([9])));
    }

    #[test]
    fn test_filter_row_incomplete() {
        // Test first row.
        let mut values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 0, 0];
        for _ in 0..72 {
            values.push(0);
        }

        let mut sudoku = Sudoku::new(values).unwrap();
        let _ = sudoku.filter();

        assert_eq!(sudoku.values[7].posibilities, Some(HashSet::from([8, 9])));
        assert_eq!(sudoku.values[8].posibilities, Some(HashSet::from([8, 9])));
    }

    #[test]
    fn test_is_solved() {
        #[rustfmt::skip]
        let values: Vec<u8> = vec![
            5, 3, 4, 6, 7, 8, 9, 1, 2,
            6, 7, 2, 1, 9, 5, 3, 4, 8,
            1, 9, 8, 3, 4, 2, 5, 6, 7,
            8, 5, 9, 7, 6, 1, 4, 2, 3,
            4, 2, 6, 8, 5, 3, 7, 9, 1,
            7, 1, 3, 9, 2, 4, 8, 5, 6,
            9, 6, 1, 5, 3, 7, 2, 8, 4,
            2, 8, 7, 4, 1, 9, 6, 3, 5,
            3, 4, 5, 2, 8, 6, 1, 7, 9,
        ];
        assert!(Sudoku::new(values).unwrap().is_solved());
    }

    #[test]
    fn test_from_string() {
        let s = "5,3,4,6,7,8,9,1,2,6,7,2,1,9,5,3,4,8,1,9,8,3,4,2,5,6,7,8,5,9,7,6,1,4,2,3,4,2,6,8,5,3,7,9,1,7,1,3,9,2,4,8,5,6,9,6,1,5,3,7,2,8,4,2,8,7,4,1,9,6,3,5,3,4,5,2,8,6,1,7,9";
        assert!(Sudoku::from(s).is_solved());

        #[rustfmt::skip]
        let s  = "\
            5, 3, 4, 6, 7, 8, 9, 1, 2,\
            6, 7, 2, 1, 9, 5, 3, 4, 8,\
            1, 9, 8, 3, 4, 2, 5, 6, 7,\
            8, 5, 9, 7, 6, 1, 4, 2, 3,\
            4, 2, 6, 8, 5, 3, 7, 9, 1,\
            7, 1, 3, 9, 2, 4, 8, 5, 6,\
            9, 6, 1, 5, 3, 7, 2, 8, 4,\
            2, 8, 7, 4, 1, 9, 6, 3, 5,\
            3, 4, 5, 2, 8, 6, 1, 7, 9\
        ";
        assert!(Sudoku::from(s).is_solved());

        #[rustfmt::skip]
        let s  = "\
            5,  , 4, 6, 7, 8, 9,  , 2,\
            6, 7, 2, 1, 9, 5, 3, 4, 8,\
            1, 9, 8, 3, 4, 2, 5, 6, 7,\
            8, 5, 9, 7, 6, 1, 4, 2, 3,\
            4, 2, 6, 8, 5, 3, 7, 9, 1,\
            7, 1, 3,  , 2, 4, 8, 5, 6,\
            9, 6, 1, 5, 3, 7,  , 8, 4,\
            2, 8, 7,  , 1, 9, 6, 3, 5,\
            3, 4, 5, 2, 8, 6, 1, 7, 9\
        ";
        assert!(!Sudoku::from(s).is_solved());
    }

    #[test]
    fn test_check() {
        // This particular sudoku is stuck, but a 7 can be written on the middle
        // part of the block number 6 (southwest block). The check function should
        // fix that.
        #[rustfmt::skip]
        let values: Vec<u8> = vec![
            0, 0, 0, 0, 0, 2, 7, 0, 5,
            0, 0, 7, 0, 1, 0, 0, 0, 0,
            5, 8, 0, 0, 0, 0, 4, 2, 0,
            0, 3, 5, 0, 9, 8, 0, 6, 0,
            7, 1, 0, 0, 2, 0, 0, 0, 9,
            0, 0, 0, 0, 0, 5, 0, 4, 0,
            1, 6, 8, 2, 4, 9, 5, 7, 3,
            0, 0, 0, 3, 5, 1, 0, 8, 0,
            3, 5, 2, 7, 8, 6, 9, 1, 4,
        ];

        let mut sudoku = Sudoku::new(values).unwrap();
        let _ = sudoku.filter();

        assert_eq!(
            sudoku.values[64].posibilities,
            Some(HashSet::from([7, 4, 9]))
        );

        let _ = sudoku.check();
        assert_eq!(sudoku.values[64].posibilities, Some(HashSet::from([7])));
    }

    #[test]
    fn test_solve() {
        #[rustfmt::skip]
        let values: Vec<u8> = vec![
            0, 0, 0, 0, 0, 2, 7, 0, 5,
            0, 0, 7, 0, 1, 0, 0, 0, 0,
            5, 8, 0, 0, 0, 0, 4, 2, 0,
            0, 3, 5, 0, 9, 8, 0, 6, 0,
            7, 1, 0, 0, 2, 0, 0, 0, 9,
            0, 0, 0, 0, 0, 5, 0, 4, 0,
            1, 6, 8, 2, 4, 9, 5, 7, 3,
            0, 0, 0, 3, 5, 1, 0, 8, 0,
            3, 5, 2, 7, 8, 6, 9, 1, 4,
        ];

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let _ = sudoku.try_solve();
        assert!(sudoku.is_solved());

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let _ = sudoku.solve();
        assert!(sudoku.is_solved());

        // Hard one.
        #[rustfmt::skip]
        let values: Vec<u8> = vec![
            0, 3, 0, 5, 0, 0, 0, 0, 0,
            1, 0, 0, 8, 0, 2, 0, 9, 0,
            0, 0, 9, 0, 0, 0, 4, 0, 0,
            8, 0, 0, 9, 0, 1, 0, 4, 0,
            0, 0, 0, 0, 7, 0, 0, 0, 0,
            0, 6, 0, 0, 0, 0, 0, 0, 3,
            7, 0, 0, 0, 4, 0, 0, 0, 0,
            0, 8, 0, 2, 0, 7, 6, 0, 0,
            0, 0, 0, 0, 5, 0, 0, 2, 0,
        ];

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let _ = sudoku.try_solve();
        assert!(!sudoku.is_solved());

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let _ = sudoku.solve();
        assert!(sudoku.is_solved());
    }
}
