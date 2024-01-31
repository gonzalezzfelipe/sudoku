use std::collections::HashSet;

use crate::errors::SudokuCreationError;
use crate::models::SudokuCell;
use crate::utils::{assert_values, print_sudoku, GroupIndexes};

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub cells: Vec<SudokuCell>,
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

        // Init possibilities
        let mut cells: Vec<SudokuCell> = vec![];

        for &value in values.iter() {
            cells.push(SudokuCell::new(value));
        }
        Ok(Sudoku { cells })
    }

    /// Pretty print sudoku on terminal.
    ///
    /// Try it, it looks nice.
    pub fn print(&self) {
        print_sudoku(self.clone());
    }

    /// Check whether sudoku is solved.
    pub fn is_solved(&self) -> bool {
        let full_hash_set = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        for indexes in GroupIndexes::new() {
            let mut hash_set = HashSet::new();
            for index in indexes {
                if let Some(value) = self.cells[index].value {
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
}
