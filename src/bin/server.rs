#[macro_use]
extern crate rocket;

use sudoku::server::solve;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![solve])
}
