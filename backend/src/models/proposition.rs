use crate::{
    schema::propositions,
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
#[diesel(table_name = propositions)]
#[diesel(belongs_to(Proposition))]
pub struct NewProposition<'r> {
    pub name: &'r str,
    pub credence: f32,
    pub description: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub qualifications: Vec<Option<String>>,
    pub restrictions: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(Proposition))]
#[diesel(table_name = propositions)]
pub struct Proposition {
    pub id: Uuid,
    pub name: String,
    pub credence: f32,
    pub description: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub qualifications: Vec<Option<String>>,
    pub restrictions: Vec<Option<String>>,
}

impl Proposition {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let propositions = propositions::table.load::<Proposition>(conn).expect("db connection");
        propositions
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let proposition = propositions::table.filter(propositions::id.eq(id)).first(conn).expect("db connection");
        proposition
    }
    // pub fn find_by_email(email: &str) -> Self {
    //     let conn = &mut establish_connection_pg();
    //     let proposition = propositions::table.filter(propositions::email.eq(email)).first(conn).expect("db connection");
    //     proposition
    // }
    pub fn create(proposition: NewProposition) -> Self {
        let conn = &mut establish_connection_pg();
        let proposition = NewProposition::from(proposition);
        let proposition = diesel::insert_into(propositions::table)
            .values(proposition)
            .get_result(conn).expect("db connection");
        proposition
    }
    pub fn update(id: Uuid, new_proposition: NewProposition) -> Self {
        let conn = &mut establish_connection_pg();

        let updated = NewProposition::from_existing(id, new_proposition);

        let proposition = diesel::update(propositions::table)
            .filter(propositions::id.eq(id))
            .set(updated)
            .get_result(conn).expect("db connection");
        proposition
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(propositions::table.filter(propositions::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewProposition<'_> {
    fn from(proposition: NewProposition) -> Proposition {
        let uuid = new_random_uuid_v4();
        Proposition {
            id: uuid,
            name: proposition.name.to_string(),
            credence: proposition.credence,
            description: proposition.description,
            links: proposition.links,
            qualifications: proposition.qualifications,
            restrictions: proposition.restrictions,
        }
    }
    fn from_existing(id: Uuid, proposition: NewProposition) -> Proposition {
        Proposition {
            id,
            name: proposition.name.to_string(),
            credence: proposition.credence,
            description: proposition.description,
            links: proposition.links,
            qualifications: proposition.qualifications,
            restrictions: proposition.restrictions,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewProposition<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_proposition_ct = ContentType::new("application", "x-new_proposition");
        if req.content_type() != Some(&new_proposition_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_proposition' or fallback to default.
        let limit = req.limits().get("new_proposition").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewProposition = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}