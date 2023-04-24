use crate::{
    models::sensitivity::{NewSensitivity, Sensitivity}
};
use rocket::delete;
use rocket::{post, get, put};
use rocket::serde::json::{json, Value};
use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let sensitivity = Sensitivity::find_all();

    if &sensitivity.len() > &0 {
        let sensitivity = &sensitivity;
        Some(json!(sensitivity))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let sensitivity = Sensitivity::find_by_id(uuid);
    
    Some(json!(sensitivity))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let sensitivity = Sensitivity::find_by_email(email);
    
// Some(json!(sensitivity})
// }

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Sensitivity::delete(uuid);
    
    None
}


#[post("/create", data = "<body>")]
pub fn create(body: NewSensitivity<'_>) -> Option<Value> {
    if body.name != "" {
        let new_sensitivity = body;

        let sensitivity = Sensitivity::create(new_sensitivity);

        Some(json!(sensitivity))
    } else {
        None
    }
}

#[put("/<id>", data = "<body>")]
pub async fn update(id: &str, body: NewSensitivity<'_>) -> Option<Value> {
    if body.name != "" {
        let uuid = Uuid::parse_str(id).expect("parse uuid");
        let sensitivity = Sensitivity::update(uuid, body);
        Some(json!(sensitivity))
    } else {
        None
    }
}