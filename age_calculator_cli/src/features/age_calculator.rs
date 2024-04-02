use crate::features::handle_error::throw_error;
use chrono::{Local, Datelike};

fn parse_string (_value: String, error: &str) -> Option<i32> {
    return match _value.trim().parse::<i32>() {
        Ok(value) => Some(value),
        Err(_) => { 
            throw_error(error); 
            None
        }
    };
}

fn month_rules (month: u32) -> bool {
    if month < 1 || month > 12 {
        throw_error("Mês precisa estar entre 1 e 12");
        return false
    }
    return true
}

pub fn calcule (_year: String, _month: String) -> Option<u32> {
    let current_date = Local::now();
    let year = parse_string(_year, "Escolha um ano válido").unwrap();
    let month = parse_string(_month, "Escolha um mês válido").unwrap() as u32;
    let mut age: u32;

    if !month_rules(month) {
        return None
    }

    age = (current_date.year() - year) as u32;

    if current_date.month() < month {
        age = age - 1;
    }

    return Some(age)
}