use crate::{
    models::preference::{NewPreference, Preference}
};
use rocket::delete;
use rocket::{post, get, put};
use rocket::serde::json::{json, Value};
use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let preferences = Preference::find_all();

    if &preferences.len() > &0 {
        let preference = &preferences[0];
        Some(json!(preference))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let preference = Preference::find_by_id(uuid);
    
    Some(json!(preference))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let preference = Preference::find_by_email(email);
    
// Some(json!(preference))
// }

#[get("/delete")]
pub fn delete_page() -> Option<Value> {
    let preferences = Preference::find_all();

    if &preferences.len() > &0 {
        let preference = &preferences;
        Some(json!(preference))
    } else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Preference::delete(uuid);
    
    None
}

#[post("/create", data = "<body>")]
pub fn create(body: NewPreference<'_>) -> Option<Value> {
    if body.display_name != "" {
        let new_preference = body;

        let preference = Preference::create(new_preference);

        Some(json!(preference))
    } else {
        None
    }
}

#[put("/<id>", data = "<body>")]
pub async fn update(id: &str, body: NewPreference<'_>) -> Option<Value> {
    if id != "" {
        let uuid = Uuid::parse_str(id).expect("parse uuid");
        let new_preference = body;

        let preference = Preference::update(uuid, new_preference);

        Some(json!(preference))
    } else {
        None
    }
}