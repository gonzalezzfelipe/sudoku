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

    // Solve and clone
    match solver.solve(sudoku.clone()) {
        Ok(solved) => solved.print(),
        Err(_) => panic!("Failed to solve sudoku."),
    };

    // Solve in place
    match solver.solve_in_place(&mut sudoku) {
        Ok(()) => sudoku.print(),
        Err(_) => panic!("Failed to solve sudoku."),
    };
}
