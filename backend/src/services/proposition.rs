use crate::{
    models::proposition::{NewProposition, Proposition}
};
use rocket::serde::json::{json, Value};
use rocket::delete;
use rocket::{post, get};
use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let propositions = Proposition::find_all();

    if &propositions.len() > &0 {
        let proposition = &propositions[0];
        Some(json!(proposition))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let proposition = Proposition::find_by_id(uuid);
    
    Some(json!(proposition))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let proposition = Proposition::find_by_email(email);
    
// Some(json!(proposition))
// }


#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Proposition::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_proposition: web::Json,
// ) -> Option<Value> {
//     let new_proposition = Proposition::update(id.into_inner(), new_proposition.into_inner());
// }


#[post("/create", data = "<body>")]
pub fn create(body: NewProposition<'_>) -> Option<Value> {
    if body.name != "" {
        let new_proposition = body;

        let proposition = Proposition::create(new_proposition);

        Some(json!(proposition))
    } else {
        None
    }
}
