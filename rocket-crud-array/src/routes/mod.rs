pub mod message;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    let mut all_routes = Vec::new();
    all_routes.extend(message::get_routes());
    return all_routes;
}