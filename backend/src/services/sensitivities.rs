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


#[post("/create", data = "<sensitivities>")]
pub fn create(sensitivities: Form<NewSensitivity>) -> Option<Value> {
    if sensitivities.name != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_sensitivities = Sensitivity {
            id: uuid,
            name: sensitivities.name.to_owned(),
            description: sensitivities.description.to_owned(),
            links: sensitivities.links.to_owned(),
            precautions: sensitivities.precautions.to_owned(),
            imparing: sensitivities.imparing.to_owned(),
            life_threatening: sensitivities.life_threatening.to_owned(),
            dietary: sensitivities.dietary.to_owned(),
            environmental: sensitivities.environmental.to_owned(),
            social: sensitivities.social.to_owned(),
        };

        diesel::insert_into(self::schema::sensitivities::dsl::sensitivities)
            .values(&new_sensitivities)
            .execute(connection)
            .expect("Error saving new sensitivities");

        Some(json!(new_sensitivities))
    } else {
        None
    }
}
