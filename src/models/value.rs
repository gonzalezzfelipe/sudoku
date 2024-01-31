use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct SudokuValue {
    pub value: Option<u8>,
    pub posibilities: Option<HashSet<u8>>,
    pub is_original_value: bool,
}

impl SudokuValue {
    pub fn new(val: u8) -> SudokuValue {
        let value = if val > 0 { Some(val) } else { None };
        let posibilities = if val > 0 {
            None
        } else {
            Some(HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
        };
        let is_original_value = val > 0;
        SudokuValue {
            value,
            posibilities,
            is_original_value,
        }
    }

    pub fn set_value(&mut self, value: u8) -> () {
        match self.value {
            Some(_) => panic!("Value already set"),
            None => self.value = Some(value),
        }
        self.posibilities = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_zero() {
        let value = SudokuValue::new(0);
        assert_eq!(value.value, None);
        assert_eq!(
            value.posibilities,
            Some(HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]))
        );
        assert!(!value.is_original_value);
    }

    #[test]
    fn test_new_non_zero() {
        let value = SudokuValue::new(1);
        assert_eq!(value.value, Some(1));
        assert_eq!(value.posibilities, None);
        assert!(value.is_original_value);
    }
}
