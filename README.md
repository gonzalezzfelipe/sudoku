# Sudoku solver for Rust

I just made this to learn rust. It is probably a lot worse than it could be,
given that I did it to learn the language.

## Usage

```rs
use sudoku::{Sudoku, SudokuSolver};

fn main() {
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
    let mut sudoku = Sudoku::new(values).expect("Invalid sudoku");
    let mut solver = SudokuSolver::new();

    // Show unsolved sudoku.
    sudoku.print();

    // Return cloned solution.
    match solver.solve(&sudoku) {
        Ok(solved) => {
            println!("Unsolved sudoku:");
            sudoku.print();
            println!("Solved sudoku:");
            solved.print()
        }
        Err(_) => panic!("Failed to solve sudoku."),
    };

    // Solve in place
    match solver.solve_in_place(&mut sudoku) {
        Ok(()) => {
            println!("Solved sudoku in place:");
            sudoku.print();
        }
        Err(_) => panic!("Failed to solve sudoku."),
    };
}
```
