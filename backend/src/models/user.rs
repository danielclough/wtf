use crate::{
    schema::users,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub address: Vec<Option<String>>,
    pub address_verified: Vec<Option<bool>>,
    pub email: Vec<Option<String>>,
    pub email_verified: Vec<Option<bool>>,
    pub phone: Vec<Option<String>>,
    pub phone_verified: Vec<Option<bool>>,
    pub taint: String,
    pub login_ids: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(User))]
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
    // pub fn find_by_email(email: &str) -> Self {
    //     let conn = &mut establish_connection_pg();
    //     let user = users::table.filter(users::email.eq(email)).first(conn).expect("db connection");
    //     user
    // }
    pub fn create(user: NewUser) -> Self {
        let conn = &mut establish_connection_pg();
        let user = NewUser::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(conn).expect("db connection");
        user
    }
    pub fn update(id: Uuid, user: User) -> Self {
        let conn = &mut establish_connection_pg();
        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set(user)
            .get_result(conn).expect("db connection");
        user
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(users::table.filter(users::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewUser {
    fn from(user: NewUser) -> User {
        let uuid = new_random_uuid_v4();
        User {
            id: uuid,
            first_name: user.first_name,
            last_name: user.last_name,
            address: user.address,
            address_verified: user.address_verified,
            email: user.email,
            email_verified: user.email_verified,
            phone: user.phone,
            phone_verified: user.phone_verified,
            taint: user.taint,
            login_ids: user.login_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}
