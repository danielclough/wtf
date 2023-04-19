use crate::{
    schema::propositions,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = propositions)]
#[diesel(belongs_to(Argument))]
pub struct NewProposition {
    pub name: String,
    pub credence: f32,
    pub description: Vec<Option<String>>,
    pub links: Vec<Option<String>>,
    pub qualifications: Vec<Option<String>>,
    pub restrictions: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(Argument))]
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
    pub fn update(id: Uuid, proposition: Proposition) -> Self {
        let conn = &mut establish_connection_pg();
        let proposition = diesel::update(propositions::table)
            .filter(propositions::id.eq(id))
            .set(proposition)
            .get_result(conn).expect("db connection");
        proposition
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(propositions::table.filter(propositions::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewProposition {
    fn from(proposition: NewProposition) -> Proposition {
        let uuid = new_random_uuid_v4();
        Proposition {
            id: uuid,
            name: proposition.name,
            credence: proposition.credence,
            description: proposition.description,
            links: proposition.links,
            qualifications: proposition.qualifications,
            restrictions: proposition.restrictions,
        }
    }
}
