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

    pub fn remove_posibility(&mut self, value: u8) -> bool {
        if let Some(mut posibilities) = self.posibilities.take() {
            let removed = posibilities.remove(&value);

            if posibilities.is_empty() {
                panic!("No more posibilities.")
            }

            if posibilities.len() == 1 {
                for &item in posibilities.iter() {
                    self.set_value(item);
                    return true;
                }
            }

            self.posibilities = Some(posibilities);
            return removed;
        }
        false
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        unimplemented!();
    }
    fn test_new_zero() {
        let value = SudokuValue::new(0);
        assert!(value.value, None);
        assert
    }
}
