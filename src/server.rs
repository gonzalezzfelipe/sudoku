use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::{models::Sudoku, SudokuSolver};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SudokuInput {
    values: Vec<i8>,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SudokuOutput {
    values: Option<Vec<i8>>,
    error: bool,
}

#[post("/", data = "<input>")]
pub fn solve(input: Json<SudokuInput>) -> Json<SudokuOutput> {
    let values: Vec<u8> = input
        .values
        .clone()
        .into_iter()
        .map(|x| x.try_into().unwrap())
        .collect();
    let mut sudoku: Sudoku = match Sudoku::new(values) {
        Ok(sudoku) => sudoku,
        Err(_) => {
            return Json(SudokuOutput {
                values: None,
                error: true,
            })
        }
    };
    let mut solver: SudokuSolver = SudokuSolver::new();
    match solver.solve_in_place(&mut sudoku) {
        Ok(()) => {
            let mut values: Vec<i8> = vec![];
            for (index, cell) in sudoku.cells.into_iter().enumerate() {
                values.insert(
                    index,
                    match cell.value {
                        Some(value) => value.try_into().unwrap(),
                        None => 0,
                    },
                )
            }

            Json(SudokuOutput {
                values: Some(values),
                error: false,
            })
        }
        Err(_) => Json(SudokuOutput {
            values: None,
            error: true,
        }),
    }
}
