use crate::{
    models::event::{NewEvent, Event},
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
    let events = Event::find_all();

    if &events.len() > &0 {
        let event = &events[0];
        Some(json!(event))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let event = Event::find_by_id(uuid);
    
    Some(json!(event))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let event = Event::find_by_email(email);
    
// Some(json!(event)
// }

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Event::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_event: web::Json,
// ) -> Option<Value> {
//     let new_event = Event::update(id.into_inner(), new_event.into_inner());
// }

#[post("/create", data = "<event>")]
pub fn create(event: Form<NewEvent>) -> Option<Value> {
    if event.name != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_event = Event {
            id: uuid,
            name: event.name.to_owned(),
            description: event.description.to_owned(),
            imgs: event.imgs.to_owned(),
            links: event.links.to_owned(),
            ticketing: event.ticketing.to_owned(),
            location: event.location.to_owned(),
            directions: event.directions.to_owned(),
            map_images: event.map_images.to_owned(),
            start_time: event.start_time.to_owned(),
            end_time: event.end_time.to_owned(),
            conduct_code_ids: event.conduct_code_ids.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some")).expect("uuid"))).collect(),
            other_expectations: event.other_expectations.to_owned(),
            account_ids: event.account_ids.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some")).expect("uuid"))).collect(),
            sensitivity_ids: event.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.to_owned().expect("some")).expect("uuid"))).collect(),
        };

        diesel::insert_into(self::schema::events::dsl::events)
            .values(&new_event)
            .execute(connection)
            .expect("Error saving new event");

        Some(json!(new_event))
    } else {
        None
    }
}
