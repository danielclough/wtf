use crate::{
    schema::roles,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub title: String,
    pub description: String,
    pub responsibility: String,
    pub discount: String,
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
    pub fn update(id: Uuid, role: Role) -> Self {
        let conn = &mut establish_connection_pg();
        let role = diesel::update(roles::table)
            .filter(roles::id.eq(id))
            .set(role)
            .get_result(conn).expect("db connection");
        role
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(roles::table.filter(roles::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewRole {
    fn from(role: NewRole) -> Role {
        let uuid = new_random_uuid_v4();
        Role {
            id: uuid,
            title: role.title,
            description: role.description,
            responsibility: role.responsibility,
            discount: role.discount,
            seen_by_role: role.seen_by_role.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}
