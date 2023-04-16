use crate::{
    models::user::{NewUser, User},
    schema,
    utils::pg::establish_connection_pg
};
use diesel::prelude::*;

use rocket::serde::json::{json, Value};

use rocket::{form::Form, delete};
use rocket::{post, get};

use uuid::{Builder, Uuid};
use rand::prelude::*;

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

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let user = User::find_by_email(email);
    
// Some(json!(user})
// }


#[post("/create", data = "<user>")]
pub fn create(user: Form<NewUser>) -> Option<Value> {
    if user.first_name != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_user = User {
            id: uuid,
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            address: user.address.to_owned(),
            address_verified: vec![Some(false)],
            email: user.email.to_owned(),
            email_verified: vec![Some(false)],
            phone: user.phone.to_owned(),
            phone_verified: vec![Some(false)],
            taint: user.taint.to_owned(),
            login_ids: vec![Some(uuid)],
        };

        diesel::insert_into(self::schema::users::dsl::users)
            .values(&new_user)
            .execute(connection)
            .expect("Error saving new user");

        Some(json!(new_user))
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

