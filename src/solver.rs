use std::collections::{HashMap, HashSet, VecDeque};

use crate::errors::{NoMoreGuessesError, UnsolvableSudokuError};
use crate::models::{Guess, Sudoku, SudokuCell};
use crate::utils::GroupIndexes;

#[derive(Debug)]
pub struct SudokuSolver {
    pub guesses: VecDeque<Guess>,
    pub used_guesses: HashSet<Guess>,
}

impl SudokuSolver {
    pub fn new() -> SudokuSolver {
        SudokuSolver {
            guesses: VecDeque::new(),
            used_guesses: HashSet::new(),
        }
    }

    fn filter(sudoku: &mut Sudoku) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = false;
        for indexes in GroupIndexes::new() {
            let mut values_in_indexes: HashSet<u8> = HashSet::new();

            for &index in &indexes {
                if let Some(value) = sudoku.cells[index].value {
                    values_in_indexes.insert(value);
                }
            }

            for &index in &indexes {
                if sudoku.cells[index].value == None {
                    let mut possibilities =
                        sudoku.cells[index].possibilities.as_ref().unwrap().clone();
                    for item in values_in_indexes.iter() {
                        modified |= possibilities.remove(item);
                    }

                    if possibilities.is_empty() {
                        return Err(UnsolvableSudokuError);
                    }
                    sudoku.cells[index].possibilities = Some(possibilities);
                }
            }
        }
        Ok(modified)
    }

    fn check(sudoku: &mut Sudoku) -> Result<bool, UnsolvableSudokuError> {
        let mut modified = false;
        for indexes in GroupIndexes::new() {
            let mut values_in_indexes_counter: HashMap<u8, i32> = HashMap::new();

            // Populate counter of possibilities in the indexes. i.e: Count how many times
            // a 7 appears in a row, or column, or block.
            for &index in &indexes {
                if sudoku.cells[index].value == None {
                    if let Some(possibilities) = sudoku.cells[index].possibilities.clone() {
                        for &possibility in possibilities.iter() {
                            let count = values_in_indexes_counter.entry(possibility).or_insert(0);
                            *count += 1;
                        }
                    } else {
                        return Err(UnsolvableSudokuError);
                    }
                }
            }

            for &index in &indexes {
                if sudoku.cells[index].value == None {
                    // If there is a index with a possibility that only occurs once, then
                    // we want to set that value. ie: You can only put a 7 in a specific
                    // place in a row.
                    for &possibility in sudoku.cells[index].possibilities.clone().unwrap().iter() {
                        if let Some(&count) = &values_in_indexes_counter.get(&possibility) {
                            if count == 1 {
                                sudoku.cells[index].possibilities =
                                    Some(HashSet::from([possibility]));
                                modified = true;
                            }
                        }
                    }
                }
            }
        }
        Ok(modified)
    }

    fn clean_possibilities(sudoku: &mut Sudoku) {
        for index in 0..81 {
            if sudoku.cells[index].value == None {
                let possibilities = sudoku.cells[index].possibilities.as_ref().unwrap().clone();
                if possibilities.len() == 1 {
                    for item in possibilities.iter() {
                        sudoku.cells[index].set_value(*item)
                    }
                }
            }
        }
    }

    fn apply_guess(&mut self, sudoku: &mut Sudoku, guess: &Guess) {
        self.used_guesses.insert(guess.clone());
        sudoku.cells = guess.state.clone();
        sudoku.cells[guess.index] = SudokuCell {
            value: Some(guess.value),
            possibilities: None,
            is_original_value: false,
        };
    }

    fn reverse_guess(sudoku: &mut Sudoku, guess: &Guess) {
        sudoku.cells = guess.state.clone();
    }

    fn produce_guess(&mut self, sudoku: &mut Sudoku) -> Option<Guess> {
        for index in 0..81 {
            if sudoku.cells[index].value == None {
                let value_to_guess = sudoku.cells[index].clone();

                // Copy possibilities, and extract one.
                let mut guess_possibilities = value_to_guess.possibilities.unwrap().clone();
                let value = guess_possibilities.iter().next().unwrap().clone();
                guess_possibilities.remove(&value);
                let guess = Guess {
                    index,
                    value,
                    other_possibilities: guess_possibilities,
                    state: sudoku.cells.clone(),
                };

                if !self.used_guesses.contains(&guess) {
                    return Some(guess);
                }
            }
        }
        None
    }

    fn guess(&mut self, sudoku: &mut Sudoku) -> Result<(), NoMoreGuessesError> {
        // Choose place to start guessing.
        match self.produce_guess(sudoku) {
            Some(guess) => {
                self.apply_guess(sudoku, &guess);
                self.guesses.push_back(guess);
                Ok(())
            }
            None => Err(NoMoreGuessesError),
        }
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

            SudokuSolver::clean_possibilities(sudoku);
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

    /// Solve sudoku in place.
    ///
    /// Will run the try_solve function until it comes to a stop, not being able to write a new
    /// cell without guessing. After that, guess a new value, and continue. If that leads to an
    /// error, then go back an try a new guess. The function doesn't returns an empty result if
    /// successful, because the sudoku is mutated and solved.
    pub fn solve_in_place(&mut self, sudoku: &mut Sudoku) -> Result<(), UnsolvableSudokuError> {
        self.used_guesses = HashSet::new();
        let mut is_solved = false;
        let mut iterations = 0;

        while !is_solved && iterations < 999 {
            match SudokuSolver::try_solve(sudoku) {
                Ok(()) => {
                    is_solved = sudoku.is_solved();
                    if is_solved {
                        self.used_guesses = HashSet::new();
                        return Ok(());
                    }
                    match self.guess(sudoku) {
                        Ok(()) => {}
                        Err(_) => {
                            self.used_guesses = HashSet::new();
                            return Err(UnsolvableSudokuError);
                        }
                    };
                }
                Err(_) => match self.guesses.pop_back() {
                    Some(guess) => match guess.other_guess() {
                        Some(new_guess) => {
                            self.apply_guess(sudoku, &new_guess);
                            self.guesses.push_back(new_guess);
                        }
                        None => {
                            SudokuSolver::reverse_guess(sudoku, &guess);
                            match self.guess(sudoku) {
                                Ok(()) => {}
                                Err(_) => return Err(UnsolvableSudokuError),
                            };
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

    /// Return solved sudoku, if possible.
    ///
    /// Will run the try_solve function until it comes to a stop, not being able to write a new
    /// cell without guessing. After that, guess a new value, and continue. If that leads to an
    /// error, then go back an try a new guess.
    pub fn solve(&mut self, sudoku: &Sudoku) -> Result<Sudoku, UnsolvableSudokuError> {
        let mut mutable_sudoku = sudoku.clone();
        match self.solve_in_place(&mut mutable_sudoku) {
            Ok(()) => Ok(mutable_sudoku),
            Err(err) => Err(err),
        }
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

        assert_eq!(sudoku.cells[8].possibilities, Some(HashSet::from([9])));
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

        assert_eq!(sudoku.cells[7].possibilities, Some(HashSet::from([8, 9])));
        assert_eq!(sudoku.cells[8].possibilities, Some(HashSet::from([8, 9])));
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
            sudoku.cells[64].possibilities,
            Some(HashSet::from([7, 4, 9]))
        );

        let _ = SudokuSolver::check(&mut sudoku);
        assert_eq!(sudoku.cells[64].possibilities, Some(HashSet::from([7])));
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
        let _ = solver.solve_in_place(&mut sudoku);
        assert!(sudoku.is_solved());

        // Hard one.
        // #[rustfmt::skip]
        let values: Vec<u8> = vec![
            0, 3, 0, 5, 0, 0, 0, 0, 0, 1, 0, 0, 8, 0, 2, 0, 9, 0, 0, 0, 9, 0, 0, 0, 4, 0, 0, 8, 0,
            0, 9, 0, 1, 0, 4, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 3, 7, 0, 0, 0,
            4, 0, 0, 0, 0, 0, 8, 0, 2, 0, 7, 6, 0, 0, 0, 0, 0, 0, 5, 0, 0, 2, 0,
        ];

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let _ = SudokuSolver::try_solve(&mut sudoku);
        assert!(!sudoku.is_solved());

        let mut sudoku = Sudoku::new(values.clone()).unwrap();
        let mut solver = SudokuSolver::new();
        let _ = solver.solve_in_place(&mut sudoku);
        assert!(sudoku.is_solved());
    }
}
