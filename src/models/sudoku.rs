use std::collections::{HashMap, HashSet};

use crate::models::value::SudokuValue;
use crate::utils::print_sudoku;

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub values: Vec<SudokuValue>,
}

impl Sudoku {
    pub fn new(values: Vec<u8>) -> Sudoku {
        // Init posibilities
        let mut parsed_values: Vec<SudokuValue> = vec![];

        for &value in values.iter() {
            parsed_values.push(SudokuValue::new(value));
        }
        Sudoku {
            values: parsed_values,
        }
    }

    pub fn print(&self) {
        print_sudoku(self.clone());
    }

    pub fn is_solved(&self) -> bool {
        let full_hash_set = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut row = 0;
        while row < 9 {
            let indexes = Sudoku::get_row_indexes(row);
            let mut row_hash_set = HashSet::new();
            for index in indexes {
                if let Some(value) = self.values[index].value {
                    row_hash_set.insert(value);
                } else {
                    return false;
                }
            }
            if full_hash_set != row_hash_set {
                return false;
            }
            row += 1;
        }

        let mut column = 0;
        while column < 9 {
            let indexes = Sudoku::get_column_indexes(column);
            let mut column_hash_set = HashSet::new();
            for index in indexes {
                if let Some(value) = self.values[index].value {
                    column_hash_set.insert(value);
                } else {
                    return false;
                }
            }
            if full_hash_set != column_hash_set {
                return false;
            }
            column += 1;
        }

        let mut block = 0;
        while block < 9 {
            let indexes = Sudoku::get_block_indexes(block);
            let mut block_hash_set = HashSet::new();
            for index in indexes {
                if let Some(value) = self.values[index].value {
                    block_hash_set.insert(value);
                } else {
                    return false;
                }
            }
            if full_hash_set != block_hash_set {
                return false;
            }
            block += 1;
        }

        true
    }

    pub fn filter(&mut self, indexes: Vec<usize>) -> bool {
        let mut modified = false;
        let mut values_in_indexes: HashSet<u8> = HashSet::new();

        for &index in &indexes {
            if let Some(value) = self.values[index].value {
                values_in_indexes.insert(value);
            }
        }

        for &index in &indexes {
            if self.values[index].value == None {
                let mut posibilities = self.values[index].posibilities.as_ref().unwrap().clone();
                for item in values_in_indexes.iter() {
                    modified |= posibilities.remove(item);
                }

                if posibilities.is_empty() {
                    panic!("No posibilities on index {}", index)
                }

                if posibilities.len() == 1 {
                    for item in posibilities.iter() {
                        self.values[index].set_value(*item)
                    }
                } else {
                    self.values[index].posibilities = Some(posibilities);
                }
            }
        }
        modified
    }

    fn check(&mut self, indexes: Vec<usize>) -> bool {
        let mut modified = false;
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
                    panic!("No posibilities and no value on index {}", index)
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
                            self.values[index].set_value(posibility);
                            // After setting a value, we must update the posibilities of the
                            // rest. Took me a while to figure this one out.
                            self.run_filters();
                            modified = true;
                        }
                    }
                }
            }
        }
        modified
    }

    pub fn get_row_indexes(row: usize) -> Vec<usize> {
        (row * 9..(row + 1) * 9).collect()
    }

    pub fn get_column_indexes(column: usize) -> Vec<usize> {
        (column..81).step_by(9).collect()
    }

    pub fn get_block_indexes(block: usize) -> Vec<usize> {
        let mut indexes: Vec<usize> = vec![];

        let mut i = 0;
        while i < 3 {
            let mut j = 0;
            while j < 3 {
                indexes.push(9 * 3 * (block / 3) + (block % 3) * 3 + i * 9 + j);
                j += 1;
            }
            i += 1;
        }
        indexes
    }

    pub fn filter_rows(&mut self) -> bool {
        let mut row = 0;
        let mut modified = false;
        while row < 9 {
            modified |= self.filter(Sudoku::get_row_indexes(row));
            row += 1;
        }
        modified
    }

    pub fn filter_columns(&mut self) -> bool {
        let mut modified = false;
        let mut column = 0;
        while column < 9 {
            modified |= self.filter(Sudoku::get_column_indexes(column));
            column += 1;
        }
        modified
    }

    pub fn filter_blocks(&mut self) -> bool {
        let mut modified = false;
        let mut block = 0;
        while block < 9 {
            modified |= self.filter(Sudoku::get_block_indexes(block));
            block += 1;
        }
        modified
    }

    pub fn check_rows(&mut self) -> bool {
        let mut modified = false;
        let mut row = 0;
        while row < 9 {
            modified |= self.check(Sudoku::get_row_indexes(row));
            row += 1;
        }
        modified
    }

    pub fn check_columns(&mut self) -> bool {
        let mut modified = false;
        let mut column = 0;
        while column < 9 {
            modified |= self.check(Sudoku::get_column_indexes(column));
            column += 1;
        }
        modified
    }

    pub fn check_blocks(&mut self) -> bool {
        let mut modified = false;
        let mut block = 0;
        while block < 9 {
            modified |= self.check(Sudoku::get_block_indexes(block));
            block += 1;
        }
        modified
    }

    pub fn run_filters(&mut self) {
        let mut modified = true;

        while modified {
            let mut internal_modified = false;
            internal_modified |= self.filter_columns();
            internal_modified |= self.filter_rows();
            internal_modified |= self.filter_blocks();

            modified = internal_modified;
        }
    }

    pub fn run_routine(&mut self) -> bool {
        let mut modified = false;

        self.run_filters();

        modified |= self.check_rows();

        modified |= self.check_columns();
        modified |= self.check_blocks();
        modified |= self.filter_columns();
        modified |= self.filter_rows();
        modified |= self.filter_blocks();
        modified
    }

    pub fn solve(&mut self) {
        let mut modified = true;
        while modified {
            modified = self.run_routine();
        }
    }
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

        Sudoku::new(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let values: Vec<u8> = vec![
            0, 0, 0, 0, 0, 2, 7, 0, 5, 0, 0, 7, 0, 1, 0, 0, 0, 0, 5, 8, 0, 0, 0, 0, 4, 2, 0, 0, 3,
            5, 0, 9, 8, 0, 0, 0, 7, 1, 0, 0, 2, 0, 0, 0, 9, 0, 0, 0, 0, 0, 5, 0, 4, 0, 1, 6, 8, 2,
            0, 9, 5, 0, 3, 0, 0, 0, 3, 0, 1, 0, 8, 0, 0, 5, 2, 7, 0, 6, 9, 1, 4,
        ];
        Sudoku::new(values);
    }

    #[test]
    fn test_get_block_indexes() {
        assert_eq!(
            vec![0, 1, 2, 9, 10, 11, 18, 19, 20],
            Sudoku::get_block_indexes(0)
        );
        assert_eq!(
            vec![3, 4, 5, 12, 13, 14, 21, 22, 23],
            Sudoku::get_block_indexes(1)
        );
        assert_eq!(
            vec![6, 7, 8, 15, 16, 17, 24, 25, 26],
            Sudoku::get_block_indexes(2)
        );
        assert_eq!(
            vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
            Sudoku::get_block_indexes(3)
        );
        assert_eq!(
            vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
            Sudoku::get_block_indexes(4)
        );
        assert_eq!(
            vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
            Sudoku::get_block_indexes(5)
        );
        assert_eq!(
            vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
            Sudoku::get_block_indexes(6)
        );
        assert_eq!(
            vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
            Sudoku::get_block_indexes(7)
        );
    }

    #[test]
    fn test_get_column_indexes() {
        assert_eq!(
            vec![0, 9, 18, 27, 36, 45, 54, 63, 72],
            Sudoku::get_column_indexes(0)
        );
        assert_eq!(
            vec![1, 10, 19, 28, 37, 46, 55, 64, 73],
            Sudoku::get_column_indexes(1)
        );
        assert_eq!(
            vec![7, 16, 25, 34, 43, 52, 61, 70, 79],
            Sudoku::get_column_indexes(7)
        );
    }

    #[test]
    fn test_filter_row_complete() {
        // Test first row.
        let mut values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        for _ in 0..72 {
            values.push(0);
        }

        let mut sudoku = Sudoku::new(values);
        sudoku.filter(Sudoku::get_row_indexes(0));

        assert_eq!(sudoku.values[8].posibilities, None);
        assert_eq!(sudoku.values[8].value, Some(9));
    }

    #[test]
    fn test_filter_row_incomplete() {
        // Test first row.
        let mut values: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 0, 0];
        for _ in 0..72 {
            values.push(0);
        }

        let mut sudoku = Sudoku::new(values);
        sudoku.filter(Sudoku::get_row_indexes(0));

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
        assert!(Sudoku::new(values).is_solved());
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

        let mut sudoku = Sudoku::new(values);
        sudoku.filter_rows();
        sudoku.filter_columns();
        sudoku.filter_blocks();

        assert_eq!(sudoku.values[64].value, None);
        assert_eq!(
            sudoku.values[64].posibilities,
            Some(HashSet::from([7, 4, 9]))
        );

        sudoku.check(Sudoku::get_block_indexes(6));
        assert_eq!(sudoku.values[64].value, Some(7));
    }
}
