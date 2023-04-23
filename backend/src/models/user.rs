use crate::{
    schema::users,
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
#[diesel(table_name = users)]
#[diesel(belongs_to(Login))]
pub struct NewUser<'r> {
    pub first_name:  &'r str,
    pub last_name:  &'r str,
    pub address: Vec<Option<String>>,
    pub address_verified: Vec<Option<bool>>,
    pub email: Vec<Option<String>>,
    pub email_verified: Vec<Option<bool>>,
    pub phone: Vec<Option<String>>,
    pub phone_verified: Vec<Option<bool>>,
    pub taint:  &'r str,
    pub login_ids: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(Login))]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub address: Vec<Option<String>>,
    pub address_verified: Vec<Option<bool>>,
    pub email: Vec<Option<String>>,
    pub email_verified: Vec<Option<bool>>,
    pub phone: Vec<Option<String>>,
    pub phone_verified: Vec<Option<bool>>,
    pub taint: String,
    pub login_ids: Vec<Option<Uuid>>,
}

impl User {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let users = users::table.load::<User>(conn).expect("db connection");
        users
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let user = users::table.filter(users::id.eq(id)).first(conn).expect("db connection");
        user
    }
    pub fn find_by_login(login_id: &str) -> Self {
        let uuid = Uuid::parse_str(login_id).expect("uuid");
        let conn = &mut establish_connection_pg();
        let user = users::table.filter(users::login_ids.is_contained_by(vec![uuid])).first(conn).expect("db connection");
        user
    }
    pub fn create(user: NewUser) -> Self {
        let conn = &mut establish_connection_pg();
        let user = NewUser::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(conn).expect("db connection");
        user
    }
    pub fn update(id: Uuid, new_user: NewUser) -> Self {
        let conn = &mut establish_connection_pg();

        let updated = NewUser::from_existing(id, new_user);

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(updated)
            .get_result(conn).expect("db connection");
        user
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewUser<'_> {
    fn from(user: NewUser) -> User {
        let uuid = new_random_uuid_v4();
        User {
            id: uuid,
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            address: user.address,
            address_verified: user.address_verified,
            email: user.email,
            email_verified: user.email_verified,
            phone: user.phone,
            phone_verified: user.phone_verified,
            taint: user.taint.to_string(),
            login_ids: user.login_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
    fn from_existing(id: Uuid, user: NewUser) -> User {
        User {
            id,
            first_name: user.first_name.to_string(),
            last_name: user.last_name.to_string(),
            address: user.address,
            address_verified: user.address_verified,
            email: user.email,
            email_verified: user.email_verified,
            phone: user.phone,
            phone_verified: user.phone_verified,
            taint: user.taint.to_string(),
            login_ids: user.login_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewUser<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_user_ct = ContentType::new("application", "x-new_user");
        if req.content_type() != Some(&new_user_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_user' or fallback to default.
        let limit = req.limits().get("new_user").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewUser = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}