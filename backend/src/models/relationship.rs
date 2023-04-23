use crate::{
    schema::relationships,
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
#[diesel(table_name = relationships)]
pub struct NewRelationship<'r> {
    pub dog_cat_bird: &'r str,
    pub ignore_ids: Vec<Option<String>>,
    pub friend_ids: Vec<Option<String>>,
    pub frienenmy_ids: Vec<Option<String>>,
    pub neutral_ids: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = relationships)]
pub struct Relationship {
    pub id: Uuid,
    pub dog_cat_bird: String,
    pub ignore_ids: Vec<Option<Uuid>>,
    pub friend_ids: Vec<Option<Uuid>>,
    pub frienenmy_ids: Vec<Option<Uuid>>,
    pub neutral_ids: Vec<Option<Uuid>>,
}

impl Relationship {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let relationships = relationships::table.load::<Relationship>(conn).expect("db connection");
        relationships
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let relationship = relationships::table.filter(relationships::id.eq(id)).first(conn).expect("db connection");
        relationship
    }
    pub fn find_by_ignore(ignore_id: &str) -> Self {
        let uuid = Uuid::parse_str(ignore_id).expect("uuid");
        let conn = &mut establish_connection_pg();
        let ignore = relationships::table.filter(relationships::ignore_ids.is_contained_by(vec![uuid])).first(conn).expect("db connection");
        ignore
    }
    pub fn find_by_friend(friend_id: &str) -> Self {
        let uuid = Uuid::parse_str(friend_id).expect("uuid");
        let conn = &mut establish_connection_pg();
        let friend = relationships::table.filter(relationships::friend_ids.is_contained_by(vec![uuid])).first(conn).expect("db connection");
        friend
    }
    pub fn find_by_frienenmy(frienenmy_id: &str) -> Self {
        let uuid = Uuid::parse_str(frienenmy_id).expect("uuid");
        let conn = &mut establish_connection_pg();
        let frienenmy = relationships::table.filter(relationships::frienenmy_ids.is_contained_by(vec![uuid])).first(conn).expect("db connection");
        frienenmy
    }
    pub fn find_by_neutral(neutral_id: &str) -> Self {
        let uuid = Uuid::parse_str(neutral_id).expect("uuid");
        let conn = &mut establish_connection_pg();
        let neutral = relationships::table.filter(relationships::neutral_ids.is_contained_by(vec![uuid])).first(conn).expect("db connection");
        neutral
    }
    pub fn create(relationship: NewRelationship) -> Self {
        let conn = &mut establish_connection_pg();
        let relationship = NewRelationship::from(relationship);
        let relationship = diesel::insert_into(relationships::table)
            .values(relationship)
            .get_result(conn).expect("db connection");
        relationship
    }
    pub fn update(id: Uuid, new_relationship: NewRelationship) -> Self {
        let conn = &mut establish_connection_pg();

        let updated = NewRelationship::from_existing(id, new_relationship);

        let relationship = diesel::update(relationships::table)
            .filter(relationships::id.eq(id))
            .set(updated)
            .get_result(conn).expect("db connection");
        relationship
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(relationships::table.filter(relationships::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewRelationship<'_> {
    fn from(relationship: NewRelationship) -> Relationship {
        let uuid = new_random_uuid_v4();
        Relationship {
            id: uuid,
            dog_cat_bird: relationship.dog_cat_bird.to_string(),
            ignore_ids: relationship.ignore_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            friend_ids: relationship.friend_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            frienenmy_ids: relationship.frienenmy_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            neutral_ids: relationship.neutral_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
    fn from_existing(id: Uuid, relationship: NewRelationship) -> Relationship {
        Relationship {
            id,
            dog_cat_bird: relationship.dog_cat_bird.to_string(),
            ignore_ids: relationship.ignore_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            friend_ids: relationship.friend_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            frienenmy_ids: relationship.frienenmy_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            neutral_ids: relationship.neutral_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewRelationship<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_relationship_ct = ContentType::new("application", "x-new_relationship");
        if req.content_type() != Some(&new_relationship_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_relationship' or fallback to default.
        let limit = req.limits().get("new_relationship").unwrap_or(512.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        
        let data: NewRelationship = serde_json::from_str(string).expect("works");
        println!("{:?}", data);
        
        Success(data)
    }
}