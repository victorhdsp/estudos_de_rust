#[macro_use] extern crate rocket;

pub mod routes {
    #[get("/")]
    pub fn hello_world() -> &'static str {
        "Hello World!"
    }
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/v1/api", routes![routes::hello_world])
}