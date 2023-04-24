use crate::{
    models::login::{NewLogin, Login},
};
use rocket::serde::json::{json, Value};
use rocket::{delete};
use rocket::{post, get, put};
use uuid::Uuid;

#[post("/create", data = "<body>")]
pub fn create(body: NewLogin<'_>) -> Option<Value> {
    if body.pw_salt != "".to_string() {

        let new_login = body;

        let login = Login::create(new_login);

        Some(json!(login))
    } else {
        None
    }
}

#[get("/list")]
pub fn list() -> Option<Value> {
    let logins = Login::find_all();

    if &logins.len() > &0 {
        let login = &logins;
        Some(json!(login))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let login = Login::find_by_id(uuid);
    
    Some(json!(login))
}

#[get("/email/<email>")]
pub fn find_by_email(email: &str) -> Option<Value> {
    let login = Login::find_by_email(email);
    
    Some(json!(login))
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Login::delete(uuid);
    
    None
}

#[put("/<id>", data = "<body>")]
pub async fn update(id: &str, body: NewLogin<'_>) -> Option<Value> {
    if id != "" {
        let uuid = Uuid::parse_str(id).expect("parse uuid");
        let login = Login::update(uuid, body);
        Some(json!(login))
    } else {
        None
    }
}