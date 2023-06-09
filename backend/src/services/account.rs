use crate::{
    models::account::{NewAccount, Account}
};
use rocket::serde::json::{json, Value};
use rocket::delete;
use rocket::{post, get, put};
use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let accounts = Account::find_all();

    if &accounts.len() > &0 {
        Some(json!(accounts))
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
    let account = Account::find_by_user(user);
    
    Some(json!(account))
}

#[post("/create", data = "<body>")]
pub fn create(body: NewAccount<'_>) -> Option<Value> {
    if body.level != "" {
        let new_account = body;

        let account = Account::create(new_account);

        Some(json!(account))
    } else {
        None
    }
}

#[get("/delete")]
pub fn delete_page() -> Option<Value> {
    let accounts = Account::find_all();

    if &accounts.len() > &0 {
        let account = &accounts[0];
        Some(json!(account))
    } else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Account::delete(uuid);
    
    None
}

#[put("/<id>", data = "<body>")]
pub async fn update(id: &str, body: NewAccount<'_>) -> Option<Value> {
    if id != "" {
        let uuid = Uuid::parse_str(id).expect("parse uuid");
        let account = Account::update(uuid, body);
        Some(json!(account))
    } else {
        None
    }
}