use crate::{
    models::account::{NewAccount, Account}
};
use rocket::serde::json::{json, Value};
use diesel::prelude::*;

use rocket::delete;
use rocket::{post, get};

use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let accounts = Account::find_all();

    if &accounts.len() > &0 {
        let account = &accounts[0];
        Some(json!(account))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let account = Account::find_by_id(uuid);
    
    Some(json!(account))
}

#[get("/user/<user>")]
pub fn find_by_user(user: &str) -> Option<Value> {
    let user = Account::find_by_user(user);
    
    Some(json!(user))
}


#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Account::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_account: web::Json,
// ) -> Option<Value> {
//     let new_account = Account::update(id.into_inner(), new_account.into_inner());
// }

#[post("/create", data = "<body>")]
pub fn create(body: NewAccount<'_>) -> Option<Value> {
    if true {
        let new_account = body;

        let account = Account::create(new_account);

        Some(json!(account))
    } else {
        None
    }
}
