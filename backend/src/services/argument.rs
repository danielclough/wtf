use crate::{
    models::argument::{NewArgument, Argument},
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

#[post("/create", data = "<argument>")]
pub fn create(argument: Form<NewArgument>) -> Option<Value> {
    if argument.name != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_argument = Argument {
            id: uuid,
            name: argument.name.to_owned(),
            description: argument.description.to_owned(),
            proposition_ids: argument.proposition_ids.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some")).expect("uuid"))).collect(),
            relationship: argument.relationship.to_owned(),
        };

        diesel::insert_into(self::schema::arguments::dsl::arguments)
            .values(&new_argument)
            .execute(connection)
            .expect("Error saving new argument");

        Some(json!(new_argument))
    } else {
        None
    }
}
