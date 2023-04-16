use crate::{
    schema::sensitivities,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = sensitivities)]
pub struct NewSensitivity {
    pub name: String,
    pub description: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub precautions: Vec<Option<String>>,
    pub imparing: bool,
    pub life_threatening: bool,
    pub dietary: bool,
    pub environmental: bool,
    pub social: bool,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Event))]
#[diesel(table_name = sensitivities)]
pub struct Sensitivity {
    pub id: Uuid,
    pub name: String,
    pub description: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub precautions: Vec<Option<String>>,
    pub imparing: bool,
    pub life_threatening: bool,
    pub dietary: bool,
    pub environmental: bool,
    pub social: bool,
}

impl Sensitivity {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let sensitivities = sensitivities::table.load::<Sensitivity>(conn).expect("db connection");
        sensitivities
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let sensitivity = sensitivities::table.filter(sensitivities::id.eq(id)).first(conn).expect("db connection");
        sensitivity
    }
    // pub fn find_by_email(email: &str) -> Self {
    //     let conn = &mut establish_connection_pg();
    //     let sensitivity = sensitivities::table.filter(sensitivities::email.eq(email)).first(conn).expect("db connection");
    //     sensitivity
    // }
    pub fn create(sensitivity: NewSensitivity) -> Self {
        let conn = &mut establish_connection_pg();
        let sensitivity = NewSensitivity::from(sensitivity);
        let sensitivity = diesel::insert_into(sensitivities::table)
            .values(sensitivity)
            .get_result(conn).expect("db connection");
        sensitivity
    }
    pub fn update(id: Uuid, sensitivity: Sensitivity) -> Self {
        let conn = &mut establish_connection_pg();
        let sensitivity = diesel::update(sensitivities::table)
            .filter(sensitivities::id.eq(id))
            .set(sensitivity)
            .get_result(conn).expect("db connection");
        sensitivity
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(sensitivities::table.filter(sensitivities::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewSensitivity {
    fn from(sensitivity: NewSensitivity) -> Sensitivity {
        let uuid = new_random_uuid_v4();
        Sensitivity {
            id: uuid,
            name: sensitivity.name,
            description: sensitivity.description,
            links: sensitivity.links,
            precautions: sensitivity.precautions,
            imparing: sensitivity.imparing,
            life_threatening: sensitivity.life_threatening,
            dietary: sensitivity.dietary,
            environmental: sensitivity.environmental,
            social: sensitivity.social,
        }
    }
}
