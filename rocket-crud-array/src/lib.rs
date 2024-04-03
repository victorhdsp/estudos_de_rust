pub mod routes;

use rocket::{Build, Rocket};

pub fn start () -> Rocket<Build>{
    return rocket::build().mount("/", routes::get_routes());
}