#[macro_use]
extern crate rocket;

pub mod errors;
pub mod models;
pub mod server;
pub mod solver;
pub mod utils;

pub use crate::models::Sudoku;
pub use crate::solver::SudokuSolver;
