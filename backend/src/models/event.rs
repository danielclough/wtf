use crate::{
    schema::events,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::{request::{self, Request}, serde::json::serde_json};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::_common::Error;

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = events)]
pub struct NewEvent<'r> {
    pub name: &'r str,
    pub description: Vec<Option<String>>,
    pub imgs: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub ticketing: Vec<Option<String>>,
    pub location: Vec<Option<String>>,
    pub directions: Vec<Option<String>>,
    pub map_images: Vec<Option<String>>,
    pub start_time: &'r str,
    pub end_time: &'r str,
    pub conduct_code_ids: Vec<Option<String>>,
    pub other_expectations: Vec<Option<String>>,
    pub account_ids: Vec<Option<String>>,
    pub sensitivity_ids: Vec<Option<String>>,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = events)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: Vec<Option<String>>,
    pub imgs: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub ticketing: Vec<Option<String>>,
    pub location: Vec<Option<String>>,
    pub directions: Vec<Option<String>>,
    pub map_images: Vec<Option<String>>,
    pub start_time: String,
    pub end_time: String,
    pub conduct_code_ids: Vec<Option<Uuid>>,
    pub other_expectations: Vec<Option<String>>,
    pub account_ids: Vec<Option<Uuid>>,
    pub sensitivity_ids: Vec<Option<Uuid>>,
}


impl Event {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let events = events::table.load::<Event>(conn).expect("db connection");
        events
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let event = events::table.filter(events::id.eq(id)).first(conn).expect("db connection");
        event
    }
    // pub fn find_by_email(email: &str) -> Self {
    //     let conn = &mut establish_connection_pg();
    //     let event = events::table.filter(events::email.eq(email)).first(conn).expect("db connection");
    //     event
    // }
    pub fn create(event: NewEvent) -> Self {
        let conn = &mut establish_connection_pg();
        let event = NewEvent::from(event);
        let event = diesel::insert_into(events::table)
            .values(event)
            .get_result(conn).expect("db connection");
        event
    }
    pub fn update(id: Uuid, event: Event) -> Self {
        let conn = &mut establish_connection_pg();
        let event = diesel::update(events::table)
            .filter(events::id.eq(id))
            .set(event)
            .get_result(conn).expect("db connection");
        event
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(events::table.filter(events::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewEvent<'_> {
    fn from(event: NewEvent) -> Event {
        let uuid = new_random_uuid_v4();
        Event {
            id: uuid,
            name: event.name.to_string(),
            description: event.description,
            imgs: event.imgs,
            links: event.links,
            ticketing: event.ticketing,
            location: event.location,
            directions: event.directions,
            map_images: event.map_images,
            start_time: event.start_time.to_string(),
            end_time: event.end_time.to_string(),
            conduct_code_ids: event.conduct_code_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            other_expectations: event.other_expectations,
            account_ids: event.account_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            sensitivity_ids: event.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewEvent<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_event_ct = ContentType::new("application", "x-new_event");
        if req.content_type() != Some(&new_event_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_event' or fallback to default.
        let limit = req.limits().get("new_event").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewEvent = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}