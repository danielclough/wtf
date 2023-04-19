use crate::{
    models::proposition::{NewProposition, Proposition},
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
    let propositions = Proposition::find_all();

    if &propositions.len() > &0 {
        let proposition = &propositions[0];
        Some(json!(proposition))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let proposition = Proposition::find_by_id(uuid);
    
    Some(json!(proposition))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let proposition = Proposition::find_by_email(email);
    
// Some(json!(proposition))
// }


#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Proposition::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_proposition: web::Json,
// ) -> Option<Value> {
//     let new_proposition = Proposition::update(id.into_inner(), new_proposition.into_inner());
// }


#[post("/create", data = "<proposition>")]
pub fn create(proposition: Form<NewProposition>) -> Option<Value> {
    if proposition.name != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_proposition = Proposition {
            id: uuid,
            name: proposition.name.to_owned(),
            credence: proposition.credence.to_owned(),
            description: proposition.description.to_owned(),
            links: proposition.links.to_owned(),
            qualifications: proposition.qualifications.to_owned(),
            restrictions: proposition.restrictions.to_owned(),
        };

        diesel::insert_into(self::schema::propositions::dsl::propositions)
            .values(&new_proposition)
            .execute(connection)
            .expect("Error saving new proposition");

        Some(json!(new_proposition))
    } else {
        None
    }
}
