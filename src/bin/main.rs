use sudoku::models::Sudoku;

fn main() {
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
    let mut sudoku = Sudoku::new(values);
    sudoku.print();
    sudoku.solve();
    sudoku.print();
    println!("Sudoku is solved: {}", sudoku.is_solved());
}
