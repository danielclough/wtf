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

#[post("/create", data = "<role>")]
pub fn create(role: Form<NewRole>) -> Option<Value> {
    if role.title != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_role = Role {
            id: uuid,
            title: role.title.to_owned(),
            description: role.description.to_owned(),
            responsibility: role.responsibility.to_owned(),
            discount: role.discount.to_owned(),
            seen_by_role: role.seen_by_role.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some")).expect("uuid"))).collect(),
        };

        diesel::insert_into(self::schema::roles::dsl::roles)
            .values(&new_role)
            .execute(connection)
            .expect("Error saving new role");

        Some(json!(new_role))
    } else {
        None
    }
}
