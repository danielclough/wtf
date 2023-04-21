use crate::{
    models::sensitivities::{NewSensitivity, Sensitivity},
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
    let sensitivities = Sensitivity::find_all();

    if &sensitivities.len() > &0 {
        let sensitivities = &sensitivities[0];
        Some(json!(sensitivities))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let sensitivities = Sensitivity::find_by_id(uuid);
    
    Some(json!(sensitivities))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let sensitivities = Sensitivity::find_by_email(email);
    
// Some(json!(sensitivities})
// }

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Sensitivity::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_sensitivities: web::Json,
// ) -> Option<Value> {
//     let new_sensitivities = Sensitivity::update(id.into_inner(), new_sensitivities.into_inner());
// }


#[post("/create", data = "<body>")]
pub fn create(body: NewSensitivity<'_>) -> Option<Value> {
    if body.name != "" {
        let new_sensitivities = body;

        let sensitivities = Sensitivity::create(new_sensitivities);

        Some(json!(sensitivities))
    } else {
        None
    }
}
