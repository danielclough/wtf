use crate::{
    models::event::{NewEvent, Event}
};
use rocket::serde::json::{json, Value};
use rocket::delete;
use rocket::{post, get, put};

use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let events = Event::find_all();

    if &events.len() > &0 {
        let event = &events;
        Some(json!(event))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let event = Event::find_by_id(uuid);
    
    Some(json!(event))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let event = Event::find_by_email(email);
    
// Some(json!(event)
// }

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Event::delete(uuid);
    
    None
}

#[post("/create", data = "<body>")]
pub fn create(body: NewEvent<'_>) -> Option<Value> {
    if body.name != "" {
        let new_event = body;

        let event = Event::create(new_event);

        Some(json!(event))
    } else {
        None
    }
}

#[put("/<id>", data = "<body>")]
pub async fn update(id: &str, body: NewEvent<'_>) -> Option<Value> {
    if body.name != "" {
        let uuid = Uuid::parse_str(id).expect("parse uuid");
        let new_event = body;

        let event = Event::update(uuid, new_event);

        Some(json!(event))
    } else {
        None
    }
}