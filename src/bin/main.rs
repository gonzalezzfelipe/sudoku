use std::io;

use sudoku::Sudoku;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    // Expects [1, 2, 3, 4 ... ] with a new line at the end. Hence the slice limits
    let sudoku: Sudoku = Sudoku::from(&buffer[1..(buffer.len() - 2)]);
    sudoku.print();
    Ok(())
}
