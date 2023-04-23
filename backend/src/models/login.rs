use crate::{schema::logins, utils::{pg::establish_connection_pg, uuid::new_random_uuid_v4}};
use diesel::{prelude::*};
use rocket::{request::{self, Request}, serde::json::serde_json};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::_common::Error;

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = logins)]
pub struct NewLogin<'r> {
    pub email: &'r str,
    pub pw_salt: &'r str,
    pub pw_hash: &'r str,
    pub mfa_salt: &'r str,
    pub mfa_hash: &'r str,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
// #[diesel(belongs_to(User))]
#[diesel(table_name = logins)]
pub struct Login {
    pub id: Uuid,
    pub email: String,
    pub pw_salt: String,
    pub pw_hash: String,
    pub mfa_salt: String,
    pub mfa_hash: String,
}


impl Login {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let logins = logins::table.load::<Login>(conn).expect("db connection");
        logins
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let login = logins::table.filter(logins::id.eq(id)).first(conn).expect("db connection");
        login
    }
    pub fn find_by_email(email: &str) -> Self {
        let conn = &mut establish_connection_pg();
        let login = logins::table.filter(logins::email.eq(email)).first(conn).expect("db connection");
        login
    }
    pub fn create(login: NewLogin) -> Self {
        let conn = &mut establish_connection_pg();
        let login = NewLogin::from(login);
        let login = diesel::insert_into(logins::table)
            .values(login)
            .get_result(conn).expect("db connection");
        login
    }
    pub fn update(id: Uuid, new_login: NewLogin) -> Self {
        let conn = &mut establish_connection_pg();

        let updated = NewLogin::from_existing(id, new_login);

        let login = diesel::update(logins::table)
            .filter(logins::id.eq(id))
            .set(updated)
            .get_result(conn).expect("db connection");
        login
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(logins::table.filter(logins::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}

impl NewLogin<'_> {
    fn from(login: NewLogin) -> Login {
        let uuid = new_random_uuid_v4();
        Login {
            id: uuid,
            email: login.email.to_string(),
            pw_salt: login.pw_salt.to_string(),
            pw_hash: login.pw_hash.to_string(),
            mfa_salt: login.mfa_salt.to_string(),
            mfa_hash: login.mfa_hash.to_string(),
        }
    }
    fn from_existing(id: Uuid, login: NewLogin) -> Login {
        Login {
            id,
            email: login.email.to_string(),
            pw_salt: login.pw_salt.to_string(),
            pw_hash: login.pw_hash.to_string(),
            mfa_salt: login.mfa_salt.to_string(),
            mfa_hash: login.mfa_hash.to_string(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewLogin<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_login_ct = ContentType::new("application", "x-new_login");
        if req.content_type() != Some(&new_login_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_login' or fallback to default.
        let limit = req.limits().get("new_login").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewLogin = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}