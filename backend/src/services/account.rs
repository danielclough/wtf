use crate::{
    models::account::{NewAccount, Account},
    schema,
    utils::pg::establish_connection_pg
};
use rocket::serde::json::{json, Value};
use diesel::prelude::*;

use rocket::{form::Form, delete};
use rocket::{post, get};

use uuid::{Builder, Uuid};
use rand::prelude::*;

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

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let account = Account::find_by_email(email);
    
// Some(json!(account))
// }


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

#[post("/create", data = "<account>")]
pub fn create(account: Form<NewAccount>) -> Option<Value> {
    if account.avatar != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_account = Account {
            id: uuid,
            avatar: account.avatar.to_owned(),
            level: account.level.to_owned(),
            preference_id: Uuid::parse_str(&account.preference_id).expect("uuid"),
            role_ids: account.role_ids.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some")).expect("uuid"))).collect(),
            sensitivity_ids: account.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some")).expect("uuid"))).collect(),
            survey_results_id: Uuid::parse_str(&account.survey_results_id).expect("uuid"),
        };

        diesel::insert_into(self::schema::accounts::dsl::accounts)
            .values(&new_account)
            .execute(connection)
            .expect("Error saving new account");

        Some(json!(new_account))
    } else {
        None
    }
}
