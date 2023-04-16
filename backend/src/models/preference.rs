use crate::{
    schema::preferences,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = preferences)]
pub struct NewPreference {
    pub browser_theme: String,
    pub display_name: String,
    pub pronouns: String,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = preferences)]
pub struct Preference {
    pub id: Uuid,
    pub browser_theme: String,
    pub display_name: String,
    pub pronouns: String,
}


impl Preference {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let preferences = preferences::table.load::<Preference>(conn).expect("db connection");
        preferences
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let preference = preferences::table.filter(preferences::id.eq(id)).first(conn).expect("db connection");
        preference
    }
    // pub fn find_by_email(email: &str) -> Self {
    //     let conn = &mut establish_connection_pg();
    //     let preference = preferences::table.filter(preferences::email.eq(email)).first(conn).expect("db connection");
    //     preference
    // }
    pub fn create(preference: NewPreference) -> Self {
        let conn = &mut establish_connection_pg();
        let preference = NewPreference::from(preference);
        let preference = diesel::insert_into(preferences::table)
            .values(preference)
            .get_result(conn).expect("db connection");
        preference
    }
    pub fn update(id: Uuid, preference: NewPreference) -> Self {
        let conn = &mut establish_connection_pg();
        let preference = diesel::update(preferences::table)
            .filter(preferences::id.eq(id))
            .set(preference)
            .get_result(conn).expect("db connection");
        preference
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(preferences::table.filter(preferences::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewPreference {
    fn from(preference: NewPreference) -> Preference {
        let uuid = new_random_uuid_v4();
        Preference {
            id: uuid,
            browser_theme: preference.browser_theme,
            display_name: preference.display_name,
            pronouns: preference.pronouns,
        }
    }
}
