use crate::{
    schema::preferences,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::{request::{self, Request}, serde::json::serde_json};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::_common::Error;

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = preferences)]
#[diesel(belongs_to(Sensitivity))]
#[diesel(belongs_to(Role))]
pub struct NewPreference<'r> {
    pub browser_theme: &'r str,
    pub display_name: &'r str,
    pub pronouns: &'r str,
    pub sensitivity_ids: Vec<Option<String>>,
    pub role_ids: Vec<Option<String>>,
}


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = preferences)]
#[diesel(belongs_to(Sensitivity))]
#[diesel(belongs_to(Role))]
pub struct Preference {
    pub id: Uuid,
    pub browser_theme: String,
    pub display_name: String,
    pub pronouns: String,
    pub sensitivity_ids: Vec<Option<Uuid>>,
    pub role_ids: Vec<Option<Uuid>>,
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
    pub fn update(id: Uuid, new_preference: NewPreference) -> Self {
        let conn = &mut establish_connection_pg();

        let updated = NewPreference::from_existing(id, new_preference);

        let preference = diesel::update(preferences::table)
            .filter(preferences::id.eq(id))
            .set(updated)
            .get_result(conn).expect("db connection");
        preference
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(preferences::table.filter(preferences::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewPreference<'_> {
    fn from(preference: NewPreference) -> Preference {
        let uuid = new_random_uuid_v4();
        Preference {
            id: uuid,
            browser_theme: preference.browser_theme.to_string(),
            display_name: preference.display_name.to_string(),
            pronouns: preference.pronouns.to_string(),
            sensitivity_ids: preference.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            role_ids: preference.role_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
    fn from_existing(id: Uuid, preference: NewPreference) -> Preference {
        Preference {
            id,
            browser_theme: preference.browser_theme.to_string(),
            display_name: preference.display_name.to_string(),
            pronouns: preference.pronouns.to_string(),
            sensitivity_ids: preference.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            role_ids: preference.role_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewPreference<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_preference_ct = ContentType::new("application", "x-new_preference");
        if req.content_type() != Some(&new_preference_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_preference' or fallback to default.
        let limit = req.limits().get("new_preference").unwrap_or(256.bytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewPreference = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}