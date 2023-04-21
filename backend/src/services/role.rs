use crate::{
    models::role::{NewRole, Role},
    schema,
    utils::pg::establish_connection_pg
};
use diesel::prelude::*;

use rocket::{form::Form, delete};
use rocket::{post, get};

use rocket::serde::json::{json, Value};
use uuid::{Builder, Uuid};
use rand::prelude::*;

#[get("/list")]
pub fn list() -> Option<Value> {
    let roles = Role::find_all();

    if &roles.len() > &0 {
        let role = &roles[0];
        Some(json!(role))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let role = Role::find_by_id(uuid);
    
    Some(json!(role))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let role = Role::find_by_email(email);
    
// Some(json!(role))
// }

#[get("/delete")]
pub fn delete_page() -> Option<Value> {
    let roles = Role::find_all();

    if &roles.len() > &0 {
        let role = &roles[0];
        Some(json!(role))
    } else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Role::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_role: web::Json,
// ) -> Option<Value> {
//     let new_role = Role::update(id.into_inner(), new_role.into_inner());
// }

#[post("/create", data = "<body>")]
pub fn create(body: NewRole<'_>) -> Option<Value> {
    if body.title != "" {
        let new_role = body;

        let role = Role::create(new_role);

        Some(json!(role))
    } else {
        None
    }
}
