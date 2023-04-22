use crate::{
    models::conduct_code::{NewConductCode, ConductCode}
};
use rocket::serde::json::{json, Value};
use rocket::delete;
use rocket::{post, get};
use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let conduct_codes = ConductCode::find_all();

    if &conduct_codes.len() > &0 {
        let conduct_code = &conduct_codes[0];
        Some(json!(conduct_code))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let conduct_code = ConductCode::find_by_id(uuid);
    
    Some(json!(conduct_code))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let conduct_code = ConductCode::find_by_email(email);
    
// Some(json!(conduct_code))
// }


#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = ConductCode::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_conduct_code: web::Json,
// ) -> Option<Value> {
//     let new_conduct_code = ConductCode::update(id.into_inner(), new_conduct_code.into_inner());
// }


#[post("/create", data = "<body>")]
pub fn create(body: NewConductCode<'_>) -> Option<Value> {
    if body.name != "" {
        let new_conduct_code = body;

        let conduct_code = ConductCode::create(new_conduct_code);

        Some(json!(conduct_code))
    } else {
        None
    }
}
