#[macro_use] extern crate rocket;

use rocket_crud_array::start;

#[launch]
fn rocket () -> _ {
    start()
}