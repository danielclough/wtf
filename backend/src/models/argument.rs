use crate::{
    schema::arguments,
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
#[diesel(table_name = arguments)]
pub struct NewArgument<'r> {
    pub name: &'r str,
    pub description: Vec<Option<String>>,
    pub proposition_ids: Vec<Option<String>>,
    pub relationship: &'r str,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = arguments)]
#[diesel(belongs_to(Proposition))]
pub struct Argument {
    pub id: Uuid,
    pub name: String,
    pub description: Vec<Option<String>>,
    pub proposition_ids: Vec<Option<Uuid>>,
    pub relationship: String,
}

impl Argument {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let arguments = arguments::table.load::<Argument>(conn).expect("db connection");
        arguments
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let argument = arguments::table.filter(arguments::id.eq(id)).first(conn).expect("db connection");
        argument
    }
    pub fn create(argument: NewArgument) -> Self {
        let conn = &mut establish_connection_pg();
        let argument = NewArgument::from(argument);
        let argument = diesel::insert_into(arguments::table)
            .values(argument)
            .get_result(conn).expect("db connection");
        argument
    }
    pub fn update(id: Uuid, argument: Argument) -> Self {
        let conn = &mut establish_connection_pg();
        let argument = diesel::update(arguments::table)
            .filter(arguments::id.eq(id))
            .set(argument)
            .get_result(conn).expect("db connection");
        argument
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(arguments::table.filter(arguments::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewArgument<'_> {
    fn from(argument: NewArgument) -> Argument {
        let uuid = new_random_uuid_v4();
        Argument {
            id: uuid,
            name: argument.name.to_string(),
            description: argument.description,
            proposition_ids: argument.proposition_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            relationship: argument.relationship.to_string(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewArgument<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_argument_ct = ContentType::new("application", "x-new_argument");
        if req.content_type() != Some(&new_argument_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_argument' or fallback to default.
        let limit = req.limits().get("new_argument").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewArgument = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}