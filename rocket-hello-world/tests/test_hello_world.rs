use rocket_hello_world;

#[cfg(test)]

mod test {
    use super::rocket_hello_world::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    
    #[test]
    fn test_hello_world () {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/v1/api/").dispatch();
        
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello World!");
    }
}