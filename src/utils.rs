use crate::models::sudoku::Sudoku;
use colored::Colorize;

fn print_separator(highlight: bool) -> () {
    print!("{}", String::from("·").bold().yellow());
    let mut i = 0;
    while i < 9 {
        if highlight {
            print!("{}", String::from("---").bold().yellow());
        } else {
            print!("---");
        }
        if i % 3 == 2 || highlight {
            print!("{}", String::from("·").bold().yellow());
        } else {
            print!("·");
        }
        i += 1;
    }
    print!("\n");
}

pub fn print_sudoku(sudoku: Sudoku) -> () {
    print_separator(true);
    let mut i = 0;

    while i < 81 {
        let value = &sudoku.values[i];
        if i % 9 == 0 {
            print!("{}", String::from("|").yellow());
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
            print!("{}", String::from("|").yellow());
        } else {
            print!("|");
        }
        if i % 9 == 8 {
            print!("\n");
            let highlight = ((i + 1) / 9) % 3 == 0;
            print_separator(highlight);
        }
        i += 1;
    }
}
