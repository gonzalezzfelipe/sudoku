#[derive(Debug)]
pub struct UnsolvableSudokuError;

#[derive(Debug, Clone)]
pub enum SudokuCreationError {
    InvalidLength,
    InvalidValues,
}

#[derive(Debug)]
pub struct NoMoreGuessesError;
