use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SudokuCell {
    pub value: Option<u8>,
    pub possibilities: Option<HashSet<u8>>,
    pub is_original_value: bool,
}

/// Represents one of the 81 cell of a sudoku grid.
///
/// Each cell can either have a value, or a hashset of possibilities.
impl SudokuCell {
    pub fn new(val: u8) -> SudokuCell {
        let value = if val > 0 { Some(val) } else { None };
        let possibilities = if val > 0 {
            None
        } else {
            Some(HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
        };
        let is_original_value = val > 0;
        SudokuCell {
            value,
            possibilities,
            is_original_value,
        }
    }

    /// Set value on cell.
    ///
    /// Will panic if the value is already set.
    pub fn set_value(&mut self, value: u8) -> () {
        match self.value {
            Some(_) => panic!("Value already set"),
            None => self.value = Some(value),
        }
        self.possibilities = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_zero() {
        let value = SudokuCell::new(0);
        assert_eq!(value.value, None);
        assert_eq!(
            value.possibilities,
            Some(HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
        );
        assert!(!value.is_original_value);
    }

    #[test]
    fn test_new_non_zero() {
        let value = SudokuCell::new(1);
        assert_eq!(value.value, Some(1));
        assert_eq!(value.possibilities, None);
        assert!(value.is_original_value);
    }
}
