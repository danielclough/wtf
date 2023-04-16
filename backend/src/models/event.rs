use crate::{
    schema::events,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, FromForm, Debug)]
pub struct NewEvent {
    pub name: String,
    pub description: Vec<Option<String>>,
    pub imgs: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub ticketing: Vec<Option<String>>,
    pub location: Vec<Option<String>>,
    pub directions: Vec<Option<String>>,
    pub map_images: Vec<Option<String>>,
    pub start_time: Vec<Option<String>>,
    pub end_time: Vec<Option<String>>,
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
    pub start_time: Vec<Option<String>>,
    pub end_time: Vec<Option<String>>,
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
impl NewEvent {
    fn from(event: NewEvent) -> Event {
        let uuid = new_random_uuid_v4();
        Event {
            id: uuid,
            name: event.name,
            description: event.description,
            imgs: event.imgs,
            links: event.links,
            ticketing: event.ticketing,
            location: event.location,
            directions: event.directions,
            map_images: event.map_images,
            start_time: event.start_time,
            end_time: event.end_time,
            conduct_code_ids: event.conduct_code_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            other_expectations: event.other_expectations,
            account_ids: event.account_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            sensitivity_ids: event.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}
