use crate::{
    models::relationship::{NewRelationship, Relationship}
};
use rocket::serde::json::{json, Value};
use rocket::delete;
use rocket::{post, get};
use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let relationships = Relationship::find_all();

    if &relationships.len() > &0 {
        let relationship = &relationships;
        Some(json!(relationship))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let relationship = Relationship::find_by_id(uuid);
    
    Some(json!(relationship))
}

#[get("/ignore/<ignore>")]
pub fn find_by_ignore(ignore: &str) -> Option<Value> {
    let ignore = Relationship::find_by_ignore(ignore);
    
    Some(json!(ignore))
}

#[get("/friend/<friend>")]
pub fn find_by_friend(friend: &str) -> Option<Value> {
    let friend = Relationship::find_by_friend(friend);
    
    Some(json!(friend))
}

#[get("/frienenmy/<frienenmy>")]
pub fn find_by_frienenmy(frienenmy: &str) -> Option<Value> {
    let frienenmy = Relationship::find_by_frienenmy(frienenmy);
    
    Some(json!(frienenmy))
}

#[get("/neutral/<neutral>")]
pub fn find_by_neutral(neutral: &str) -> Option<Value> {
    let neutral = Relationship::find_by_neutral(neutral);
    
    Some(json!(neutral))
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Relationship::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_relationship: web::Json,
// ) -> Option<Value> {
//     let new_relationship = Relationship::update(id.into_inner(), new_relationship.into_inner());
// }

#[post("/create", data = "<body>")]
pub fn create(body: NewRelationship<'_>) -> Option<Value> {
    if true {
        let new_relationship = body;

        let relationship = Relationship::create(new_relationship);

        Some(json!(relationship))
    } else {
        None
    }
}
