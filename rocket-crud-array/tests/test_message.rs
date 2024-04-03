#[macro_use] extern crate rocket;

#[cfg(test)]

use rocket_crud_array::start;

#[launch]
fn rocket () -> _ { start() }

mod tests {
    use std::vec;

    use rocket_crud_array::routes::message::{clear_db, Data, Message};

    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_get_clear () {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");
        clear_db();

        let response = client.get("/").dispatch();

        let expected = Data {
            list: vec![]
        };

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_json::<Data>().unwrap(), expected);
    }

    #[test]
    fn test_get () {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");
        clear_db();
        
        let message = Message { text: "get teste" };
        client.post("/").json(&message).dispatch();

        let response = client.get("/").dispatch();

        let expected = "get teste".to_string();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_json::<Data>().unwrap().list.contains(&expected));
    }

    #[test]
    fn test_post () {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");

        let message = Message { text: "post teste" };
        let response = client.post("/").json(&message).dispatch();

        let expected = "post teste".to_string();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_json::<Data>().unwrap().list.contains(&expected));
    }

    #[test]
    fn test_put () {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");
        clear_db();

        let message_post = Message { text: "teste" };
        client.post("/").json(&message_post).dispatch();

        let message_put = Message { text: "put teste" };
        let response = client.put("/0").json(&message_put).dispatch();

        let expected = "put teste".to_string();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_json::<Data>().unwrap().list.contains(&expected));
    }

    #[test]
    fn test_delete () {
        let client = Client::tracked(super::rocket()).expect("valid rocket instance");
        clear_db();
        
        let message_post = Message { text: "teste" };
        client.post("/").json(&message_post).dispatch();

        let response = client.delete("/0").dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}