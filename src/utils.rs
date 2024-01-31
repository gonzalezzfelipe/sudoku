use std::collections::HashMap;

use colored::Colorize;

use crate::errors::SudokuCreationError;
use crate::models::Sudoku;

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

pub struct GroupIndexes {
    i: usize,
}

impl GroupIndexes {
    pub fn new() -> GroupIndexes {
        GroupIndexes { i: 0 }
    }
}

impl Iterator for GroupIndexes {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i / 9 == 0 {
            self.i += 1;
            Some(get_row_indexes(self.i - 1))
        } else if self.i / 9 == 1 {
            self.i += 1;
            Some(get_column_indexes(self.i - 1 - 9))
        } else if self.i / 9 == 2 {
            self.i += 1;
            Some(get_block_indexes(self.i - 1 - 18))
        } else {
            None
        }
    }
}

pub fn assert_values(values: Vec<u8>) -> Result<(), SudokuCreationError> {
    if values.len() != 81 {
        return Err(SudokuCreationError::InvalidLength);
    }

    for row in 0..9 {
        let indexes = get_row_indexes(row);
        let mut counter: HashMap<u8, i32> = HashMap::new();
        for index in indexes {
            if values[index] != 0 {
                let count = counter.entry(values[index]).or_insert(0);
                *count += 1;
                if *count > 1 {
                    return Err(SudokuCreationError::InvalidValues);
                }
            }
        }
    }

    for column in 0..9 {
        let indexes = get_column_indexes(column);
        let mut counter: HashMap<u8, i32> = HashMap::new();
        for index in indexes {
            if values[index] != 0 {
                let count = counter.entry(values[index]).or_insert(0);
                *count += 1;
                if *count > 1 {
                    return Err(SudokuCreationError::InvalidValues);
                }
            }
        }
    }

    for block in 0..9 {
        let indexes = get_block_indexes(block);
        let mut counter: HashMap<u8, i32> = HashMap::new();
        for index in indexes {
            if values[index] != 0 {
                let count = counter.entry(values[index]).or_insert(0);
                *count += 1;
                if *count > 1 {
                    return Err(SudokuCreationError::InvalidValues);
                }
            }
        }
    }

    Ok(())
}

pub fn print_separator(row: usize) -> () {
    // Start
    if row == 0 {
        print!("{}", String::from("╔").bold().yellow());
    } else if row == 8 {
        print!("{}", String::from("╚").bold().yellow());
    } else if row % 3 == 0 {
        print!("{}", String::from("╠").bold().yellow());
    } else {
        print!("{}", String::from("╟").bold().yellow());
    }
    for i in 0..9 {
        // Fill
        if row == 0 {
            print!("{}", String::from("═══").bold().yellow());
        } else if row == 8 {
            print!("{}", String::from("═══").bold().yellow());
        } else if row % 3 == 0 {
            print!("{}", String::from("═══").bold().yellow());
        } else {
            print!("{}", String::from("───").bold().yellow());
        }

        // Separator
        if i % 3 == 2 && i != 8 {
            if row == 0 {
                print!("{}", String::from("╦").bold().yellow());
            } else if row == 8 {
                print!("{}", String::from("╩").bold().yellow());
            } else if row % 3 == 0 {
                print!("{}", String::from("╬").bold().yellow());
            } else {
                print!("{}", String::from("╫").bold().yellow());
            }
        } else if i != 8 {
            if row == 0 {
                print!("{}", String::from("╤").bold().yellow());
            } else if row == 8 {
                print!("{}", String::from("╧").bold().yellow());
            } else if row % 3 == 0 {
                print!("{}", String::from("╪").bold().yellow());
            } else {
                print!("{}", String::from("┼").bold().yellow());
            }
        }
    }
    if row == 0 {
        print!("{}", String::from("╗").bold().yellow());
    } else if row == 8 {
        print!("{}", String::from("╝").bold().yellow());
    } else if row % 3 == 0 {
        print!("{}", String::from("╣").bold().yellow());
    } else {
        print!("{}", String::from("╢").bold().yellow());
    }
    print!("\n");
}

pub fn print_sudoku(sudoku: Sudoku) -> () {
    print_separator(0);
    let mut i = 0;

    while i < 81 {
        let value = &sudoku.cells[i];
        if i % 9 == 0 {
            print!("{}", String::from("║").yellow());
        }
        if let Some(actual_value) = value.value {
            if value.is_original_value {
                print!(
                    "{}",
                    format!(" {} ", actual_value).bold().black().on_yellow()
                );
            } else {
                print!(" {} ", actual_value);
            }
        } else {
            print!("   ");
        }
        if i % 3 == 2 {
            print!("{}", String::from("║").yellow());
        } else {
            print!("{}", String::from("│").yellow());
        }
        if i % 9 == 8 {
            print!("\n");
            let row = (i / 9) + 1;
            if row == 8 {
                print_separator(7);
            } else if row != 9 {
                print_separator(row);
            }
        }
        i += 1;
    }
    print_separator(8);
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
    fn test_get_block_indexes() {
        assert_eq!(vec![0, 1, 2, 9, 10, 11, 18, 19, 20], get_block_indexes(0));
        assert_eq!(vec![3, 4, 5, 12, 13, 14, 21, 22, 23], get_block_indexes(1));
        assert_eq!(vec![6, 7, 8, 15, 16, 17, 24, 25, 26], get_block_indexes(2));
        assert_eq!(
            vec![27, 28, 29, 36, 37, 38, 45, 46, 47],
            get_block_indexes(3)
        );
        assert_eq!(
            vec![30, 31, 32, 39, 40, 41, 48, 49, 50],
            get_block_indexes(4)
        );
        assert_eq!(
            vec![33, 34, 35, 42, 43, 44, 51, 52, 53],
            get_block_indexes(5)
        );
        assert_eq!(
            vec![54, 55, 56, 63, 64, 65, 72, 73, 74],
            get_block_indexes(6)
        );
        assert_eq!(
            vec![57, 58, 59, 66, 67, 68, 75, 76, 77],
            get_block_indexes(7)
        );
    }

    #[test]
    fn test_get_column_indexes() {
        assert_eq!(
            vec![0, 9, 18, 27, 36, 45, 54, 63, 72],
            get_column_indexes(0)
        );
        assert_eq!(
            vec![1, 10, 19, 28, 37, 46, 55, 64, 73],
            get_column_indexes(1)
        );
        assert_eq!(
            vec![7, 16, 25, 34, 43, 52, 61, 70, 79],
            get_column_indexes(7)
        );
    }

    #[test]
    fn test_group_indexes() {
        let mut i = 0;
        for _ in GroupIndexes::new() {
            i += 1;
        }
        assert_eq!(i, 27);
    }
}
