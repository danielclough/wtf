use crate::{
    models::argument::{NewArgument, Argument}
};
use rocket::serde::json::{json, Value};

use rocket::delete;
use rocket::{post, get};

use uuid::Uuid;

#[get("/list")]
pub fn list() -> Option<Value> {
    let arguments = Argument::find_all();

    if &arguments.len() > &0 {
        let argument = &arguments[0];
        Some(json!(argument))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let argument = Argument::find_by_id(uuid);
    
    Some(json!(argument))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let argument = Argument::find_by_email(email);
    
// Some(json!(argument))
// }


#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Argument::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_argument: web::Json,
// ) -> Option<Value> {
//     let new_argument = Argument::update(id.into_inner(), new_argument.into_inner());
// }

#[post("/create", data = "<body>")]
pub fn create(body: NewArgument<'_>) -> Option<Value> {
    if body.name != "" {
        let new_argument = body;

        let argument = Argument::create(new_argument);

        Some(json!(argument))
    } else {
        None
    }
}
