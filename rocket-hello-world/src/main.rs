#[macro_use] extern crate rocket;

use rocket_hello_world::rocket;

#[launch]
fn _rocket() -> _ {
    rocket()
}