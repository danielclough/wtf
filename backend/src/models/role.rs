use crate::{
    schema::roles,
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
#[diesel(table_name = roles)]
pub struct NewRole<'r> {
    pub title: &'r str,
    pub description: &'r str,
    pub responsibility: &'r str,
    pub discount: &'r str,
    pub seen_by_role: Vec<Option<String>>,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(Account))]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub responsibility: String,
    pub discount: String,
    pub seen_by_role: Vec<Option<Uuid>>,
}

impl Role {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let roles = roles::table.load::<Role>(conn).expect("db connection");
        roles
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let role = roles::table.filter(roles::id.eq(id)).first(conn).expect("db connection");
        role
    }
    // pub fn find_by_email(email: &str) -> Self {
    //     let conn = &mut establish_connection_pg();
    //     let role = roles::table.filter(roles::email.eq(email)).first(conn).expect("db connection");
    //     role
    // }
    pub fn create(role: NewRole) -> Self {
        let conn = &mut establish_connection_pg();
        let role = NewRole::from(role);
        let role = diesel::insert_into(roles::table)
            .values(role)
            .get_result(conn).expect("db connection");
        role
    }
    pub fn update(id: Uuid, new_role: NewRole) -> Self {
        let conn = &mut establish_connection_pg();

        let updated = NewRole::from_existing(id, new_role);

        let role = diesel::update(roles::table)
            .filter(roles::id.eq(id))
            .set(updated)
            .get_result(conn).expect("db connection");
        role
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(roles::table.filter(roles::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewRole<'_> {
    fn from(role: NewRole) -> Role {
        let uuid = new_random_uuid_v4();
        Role {
            id: uuid,
            title: role.title.to_string(),
            description: role.description.to_string(),
            responsibility: role.responsibility.to_string(),
            discount: role.discount.to_string(),
            seen_by_role: role.seen_by_role.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
    fn from_existing(uuid: Uuid, role: NewRole) -> Role {
        Role {
            id: uuid,
            title: role.title.to_string(),
            description: role.description.to_string(),
            responsibility: role.responsibility.to_string(),
            discount: role.discount.to_string(),
            seen_by_role: role.seen_by_role.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewRole<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_role_ct = ContentType::new("application", "x-new_role");
        if req.content_type() != Some(&new_role_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_role' or fallback to default.
        let limit = req.limits().get("new_role").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewRole = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}