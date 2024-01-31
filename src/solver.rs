use std::collections::{HashMap, HashSet, VecDeque};

use crate::errors::UnsolvableSudokuError;
use crate::models::{Guess, Sudoku, SudokuValue};
use crate::utils::GroupIndexes;

#[derive(Debug)]
pub struct SudokuSolver {
    pub guesses: VecDeque<Guess>,
}

impl SudokuSolver {
    pub fn new() -> SudokuSolver {
        SudokuSolver {
            guesses: VecDeque::new(),
        }
    }

    fn filter(sudoku: &mut Sudoku) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = false;
        for indexes in GroupIndexes::new() {
            let mut values_in_indexes: HashSet<u8> = HashSet::new();

            for &index in &indexes {
                if let Some(value) = sudoku.values[index].value {
                    values_in_indexes.insert(value);
                }
            }

            for &index in &indexes {
                if sudoku.values[index].value == None {
                    let mut posibilities =
                        sudoku.values[index].posibilities.as_ref().unwrap().clone();
                    for item in values_in_indexes.iter() {
                        modified |= posibilities.remove(item);
                    }

                    if posibilities.is_empty() {
                        return Err(UnsolvableSudokuError);
                    }
                    sudoku.values[index].posibilities = Some(posibilities);
                }
            }
        }
        Ok(modified)
    }

    fn check(sudoku: &mut Sudoku) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = false;
        for indexes in GroupIndexes::new() {
            let mut values_in_indexes_counter: HashMap<u8, i32> = HashMap::new();

            // Populate counter of posibilities in the indexes. i.e: Count how many times
            // a 7 appears in a row, or column, or block.
            for &index in &indexes {
                if sudoku.values[index].value == None {
                    if let Some(posibilities) = sudoku.values[index].posibilities.clone() {
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
                if sudoku.values[index].value == None {
                    // If there is a index with a posibility that only occurs once, then
                    // we want to set that value. ie: You can only put a 7 in a specific
                    // place in a row.
                    for &posibility in sudoku.values[index].posibilities.clone().unwrap().iter() {
                        if let Some(&count) = &values_in_indexes_counter.get(&posibility) {
                            if count == 1 {
                                sudoku.values[index].posibilities =
                                    Some(HashSet::from([posibility]));
                                modified = true;
                            }
                        }
                    }
                }
            }
        }
        Ok(modified)
    }

    fn clean_posibilities(sudoku: &mut Sudoku) {
        for index in 0..81 {
            if sudoku.values[index].value == None {
                let posibilities = sudoku.values[index].posibilities.as_ref().unwrap().clone();
                if posibilities.len() == 1 {
                    for item in posibilities.iter() {
                        sudoku.values[index].set_value(*item)
                    }
                }
            }
        }
    }

    fn apply_guess(sudoku: &mut Sudoku, guess: &Guess) {
        sudoku.values = guess.state.clone();
        sudoku.values[guess.index] = SudokuValue {
            value: Some(guess.value),
            posibilities: None,
            is_original_value: false,
        };
    }

    fn reverse_guess(sudoku: &mut Sudoku, guess: &Guess) {
        sudoku.values = guess.state.clone();
    }

    fn guess(&mut self, sudoku: &mut Sudoku) {
        // Choose place to start guessing.
        let mut index = 0;
        while sudoku.values[index].value != None {
            index += 1;
        }
        let value_to_guess = sudoku.values[index].clone();

        // Copy posibilities, and extract one.
        let mut guess_posibilities = value_to_guess.posibilities.unwrap().clone();
        let value = guess_posibilities.iter().next().unwrap().clone();
        guess_posibilities.remove(&value);
        let guess = Guess {
            index,
            value,
            other_posibilities: guess_posibilities,
            state: sudoku.values.clone(),
        };

        // Replace value in sudoku values, and retry.
        SudokuSolver::apply_guess(sudoku, &guess);
        self.guesses.push_back(guess);
    }

    fn run_routine(sudoku: &mut Sudoku) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = true;

        while modified {
            modified = false;
            modified = match SudokuSolver::filter(sudoku) {
                Ok(value) => modified || value,
                Err(err) => return Err(err),
            };
            modified = match SudokuSolver::check(sudoku) {
                Ok(value) => modified || value,
                Err(err) => return Err(err),
            };

            SudokuSolver::clean_posibilities(sudoku);
        }

        Ok(modified)
    }

    /// Try and solve the sudoku without guessing.
    ///
    /// Generally speaking, there are hard and easy sudokus. Easy sudokus can be solved
    /// without guessing. Hard sudokus are the ones that cannot be solved without guessing
    /// and checking that the sudoku is solvable.
    pub fn try_solve(sudoku: &mut Sudoku) -> Result<(), UnsolvableSudokuError> {
        let mut modified = true;
        while modified {
            modified = match SudokuSolver::run_routine(sudoku) {
                Ok(value) => value,
                Err(err) => return Err(err),
            };
        }
        Ok(())
    }

    pub fn solve(&mut self, sudoku: &mut Sudoku) -> Result<(), UnsolvableSudokuError> {
        let mut is_solved = false;
        let mut iterations = 0;

        while !is_solved && iterations < 999 {
            match SudokuSolver::try_solve(sudoku) {
                Ok(()) => {
                    is_solved = sudoku.is_solved();
                    if is_solved {
                        return Ok(());
                    }
                    self.guess(sudoku);
                }
                Err(_) => match self.guesses.pop_back() {
                    Some(guess) => match guess.other_guess() {
                        Some(new_guess) => {
                            SudokuSolver::apply_guess(sudoku, &new_guess);
                            self.guesses.push_back(new_guess);
                        }
                        None => {
                            SudokuSolver::reverse_guess(sudoku, &guess);
                            self.guess(sudoku);
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
    fn test_filter_row_complete() {
        // Test first row.
        let mut values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        for _ in 0..72 {
            values.push(0);
        }
        let mut sudoku = Sudoku::new(values).unwrap();

        let _ = SudokuSolver::filter(&mut sudoku);

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
        let _ = SudokuSolver::filter(&mut sudoku);

        assert_eq!(sudoku.values[7].posibilities, Some(HashSet::from([8, 9])));
        assert_eq!(sudoku.values[8].posibilities, Some(HashSet::from([8, 9])));
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
        let _ = SudokuSolver::filter(&mut sudoku);

        assert_eq!(
            sudoku.values[64].posibilities,
            Some(HashSet::from([7, 4, 9]))
        );

        let _ = SudokuSolver::check(&mut sudoku);
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
        let _ = SudokuSolver::try_solve(&mut sudoku);
        assert!(sudoku.is_solved());

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let mut solver = SudokuSolver::new();
        let _ = solver.solve(&mut sudoku);
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
        let _ = SudokuSolver::try_solve(&mut sudoku);
        assert!(!sudoku.is_solved());

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let mut solver = SudokuSolver::new();
        let _ = solver.solve(&mut sudoku);
        assert!(sudoku.is_solved());
    }
}
