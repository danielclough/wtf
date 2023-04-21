use crate::{
    schema::conduct_codes,
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
#[diesel(table_name = conduct_codes)]
#[diesel(belongs_to(Sensitivity))]
pub struct NewConductCode<'r> {
    pub name: &'r str,
    pub description: Vec<Option<String>>,
    pub qualifications: Vec<Option<String>>,
    pub restrictions: Vec<Option<String>>,
    pub examples: Vec<Option<String>>,
    pub sensitivity_ids: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(Sensitivity))]
#[diesel(table_name = conduct_codes)]
pub struct ConductCode {
    pub id: Uuid,
    pub name: String,
    pub description: Vec<Option<String>>,
    pub qualifications: Vec<Option<String>>,
    pub restrictions: Vec<Option<String>>,
    pub examples: Vec<Option<String>>,
    pub sensitivity_ids: Vec<Option<Uuid>>,
}

impl ConductCode {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let conduct_codes = conduct_codes::table.load::<ConductCode>(conn).expect("db connection");
        conduct_codes
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let conduct_code = conduct_codes::table.filter(conduct_codes::id.eq(id)).first(conn).expect("db connection");
        conduct_code
    }
    // pub fn find_by_email(email: &str) -> Self {
    //     let conn = &mut establish_connection_pg();
    //     let conduct_code = conduct_codes::table.filter(conduct_codes::email.eq(email)).first(conn).expect("db connection");
    //     conduct_code
    // }
    pub fn create(conduct_code: NewConductCode) -> Self {
        let conn = &mut establish_connection_pg();
        let conduct_code = NewConductCode::from(conduct_code);
        let conduct_code = diesel::insert_into(conduct_codes::table)
            .values(conduct_code)
            .get_result(conn).expect("db connection");
        conduct_code
    }
    pub fn update(id: Uuid, conduct_code: ConductCode) -> Self {
        let conn = &mut establish_connection_pg();
        let conduct_code = diesel::update(conduct_codes::table)
            .filter(conduct_codes::id.eq(id))
            .set(conduct_code)
            .get_result(conn).expect("db connection");
        conduct_code
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(conduct_codes::table.filter(conduct_codes::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewConductCode<'_> {
    fn from(conduct_code: NewConductCode) -> ConductCode {
        let uuid = new_random_uuid_v4();
        ConductCode {
            id: uuid,
            name: conduct_code.name.to_string(),
            description: conduct_code.description,
            qualifications: conduct_code.qualifications,
            restrictions: conduct_code.restrictions,
            examples: conduct_code.examples,
            sensitivity_ids: conduct_code.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some").as_str()).expect("uuid"))).collect(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewConductCode<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_conduct_code_ct = ContentType::new("application", "x-new_conduct_code");
        if req.content_type() != Some(&new_conduct_code_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_conduct_code' or fallback to default.
        let limit = req.limits().get("new_conduct_code").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewConductCode = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}