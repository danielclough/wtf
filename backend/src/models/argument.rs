use crate::{
    schema::arguments,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = arguments)]
pub struct NewArgument {
    pub name: String,
    pub description: Vec<Option<String>>,
    pub proposition_ids: Vec<Option<String>>,
    pub relationship: String,
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
impl NewArgument {
    fn from(argument: NewArgument) -> Argument {
        let uuid = new_random_uuid_v4();
        Argument {
            id: uuid,
            name: argument.name,
            description: argument.description,
            proposition_ids: argument.proposition_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            relationship: argument.relationship,
        }
    }
}
