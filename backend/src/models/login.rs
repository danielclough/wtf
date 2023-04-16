use crate::{schema::logins, utils::{pg::establish_connection_pg, uuid::new_random_uuid_v4}};
use diesel::{prelude::*};
use rocket::FromForm;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = logins)]
pub struct NewLogin {
    pub email: String,
    pub pw_salt: String,
    pub pw_hash: String,
    pub mfa_salt: String,
    pub mfa_hash: String,
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
    pub fn update(id: Uuid, login: NewLogin) -> Self {
        let conn = &mut establish_connection_pg();
        let login = diesel::update(logins::table)
            .filter(logins::id.eq(id))
            .set(login)
            .get_result(conn).expect("db connection");
        login
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(logins::table.filter(logins::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewLogin {
    fn from(login: NewLogin) -> Login {
        let uuid = new_random_uuid_v4();
        Login {
            id: uuid,
            email: login.email,
            pw_salt: login.pw_salt,
            pw_hash: login.pw_hash,
            mfa_salt: login.mfa_salt,
            mfa_hash: login.mfa_hash,
        }
    }
}