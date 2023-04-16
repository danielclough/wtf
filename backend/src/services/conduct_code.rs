use crate::{
    models::conduct_code::{NewConductCode, ConductCode},
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
    let conduct_codes = ConductCode::find_all();

    if &conduct_codes.len() > &0 {
        let conduct_code = &conduct_codes[0];
        Some(json!(conduct_code))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let conduct_code = ConductCode::find_by_id(uuid);
    
    Some(json!(conduct_code))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let conduct_code = ConductCode::find_by_email(email);
    
// Some(json!(conduct_code))
// }


#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = ConductCode::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_conduct_code: web::Json,
// ) -> Option<Value> {
//     let new_conduct_code = ConductCode::update(id.into_inner(), new_conduct_code.into_inner());
// }


#[post("/create", data = "<conduct_code>")]
pub fn create(conduct_code: Form<NewConductCode>) -> Option<Value> {
    if conduct_code.name != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_conduct_code = ConductCode {
            id: uuid,
            name: conduct_code.name.to_owned(),
            description: conduct_code.description.to_owned(),
            qualifications: conduct_code.qualifications.to_owned(),
            restrictions: conduct_code.restrictions.to_owned(),
            examples: conduct_code.examples.to_owned(),
            sensitivity_ids: conduct_code.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some").as_str()).expect("uuid"))).collect(),
        };

        diesel::insert_into(self::schema::conduct_codes::dsl::conduct_codes)
            .values(&new_conduct_code)
            .execute(connection)
            .expect("Error saving new conduct_code");

        Some(json!(new_conduct_code))
    } else {
        None
    }
}
