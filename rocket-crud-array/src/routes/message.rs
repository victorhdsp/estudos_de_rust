use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use rocket::serde::{Serialize, json::Json};
use rocket::{Route, routes};
use serde::Deserialize;

lazy_static! {
    static ref DATA: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}

unsafe fn get_data() -> std::sync::MutexGuard<'static, Vec<std::string::String>> {
    let data = DATA.lock().expect("Failed to lock mutex");
    return data;
}

pub fn clear_db () {
    unsafe {
        get_data().clear();
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Data { 
    pub list: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Message<'r> {
    pub text: &'r str
}


#[rocket::get("/")]
fn get() -> Json<Data> {
    return Json(Data {
        list: unsafe { get_data().clone() }
    })
}

#[rocket::post("/", data = "<message>")]
fn post(message: Json<Message>) -> Json<Data> {
    unsafe {
        get_data().push(message.text.to_string());
    }

    return Json(Data {
        list: unsafe { get_data().clone() }
    })
}

#[rocket::put("/<id>", data = "<message>")]
fn put(id: usize, message: Json<Message>) -> Json<Data> {
    unsafe {
        get_data()[id] = message.text.to_string()
    }

    return Json(Data {
        list: unsafe { get_data().clone() }
    })
}

#[rocket::delete("/<id>")]
fn delete(id: usize) -> () {
    unsafe {
        get_data().remove(id);
    }

    return ;
}

pub fn get_routes() -> Vec<Route> {
    routes![get, post, put, delete]
}