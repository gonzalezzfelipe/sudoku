#[derive(Debug)]
pub struct UnsolvableSudokuError;

#[derive(Debug, Clone)]
pub enum SudokuCreationError {
    InvalidLength,
    InvalidValues,
}
