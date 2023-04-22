use crate::{
    models::user::{NewUser, User}
};
use rocket::serde::json::{json, Value};
use rocket::delete;
use rocket::{post, get};
use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let users = User::find_all();

    if &users.len() > &0 {
        Some(json!(users))
    } else {
        None
    }
}

#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let user = User::find_by_id(uuid);
    
    Some(json!(user))
}

#[get("/login/<login>")]
pub fn find_by_login(login: &str) -> Option<Value> {
    let user = User::find_by_login(login);
    
    Some(json!(user))
}


#[post("/create", data = "<body>")]
pub fn create(body: NewUser<'_>) -> Option<Value> {
    if body.first_name != "" {
        let new_user = body;

        let user = User::create(new_user);

        Some(json!(user))
    } else {
        None
    }
}

#[get("/delete")]
pub fn delete_page() -> Option<Value> {
    let users = User::find_all();

    if &users.len() > &0 {
        let user = &users[0];
        Some(json!(user))
    } else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = User::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_user: web::Json,
// ) -> Option<Value> {
//     let new_user = User::update(id.into_inner(), new_user.into_inner());
// }

