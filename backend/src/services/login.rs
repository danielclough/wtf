use crate::{
    models::login::{NewLogin, Login},
};
use rocket::serde::json::{json, Value};
use rocket::{form::Form, delete};
use rocket::{post, get};
use uuid::Uuid;

#[post("/create", data = "<user_input>")]
pub fn create(user_input: Form<NewLogin>) -> Option<Value> {
    if user_input.pw_salt != "".to_string() {

        let new_login = NewLogin {
            email: user_input.email.to_string(),
            pw_salt: user_input.pw_salt.to_string(),
            pw_hash: user_input.pw_hash.to_string(),
            mfa_salt: user_input.mfa_salt.to_string(),
            mfa_hash: user_input.mfa_hash.to_string(),
        };
        let login = Login::create(new_login);

        Some(json!(login))
    } else {
        None
    }
}

#[get("/list")]
pub fn list() -> Option<Value> {
    let logins = Login::find_all();

    if &logins.len() > &0 {
        let login = &logins[0];
        Some(json!(login))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let login = Login::find_by_id(uuid);
    
    Some(json!(login))
}

#[get("/email/<email>")]
pub fn find_by_email(email: &str) -> Option<Value> {
    let login = Login::find_by_email(email);
    
    Some(json!(login))
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Login::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_login: web::Json,
// ) -> Option<Value> {
//     let new_login = Login::update(id.into_inner(), new_login.into_inner());
// }
