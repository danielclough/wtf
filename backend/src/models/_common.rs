use rocket::serde::json::serde_json::{Map, Number};

pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}


#[derive(Debug)]
pub enum Error {
    TooLarge,
    NoColon,
    InvalidAge,
    Io(std::io::Error),
}