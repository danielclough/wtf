use crate::{
    models::preference::{NewPreference, Preference},
    schema,
    utils::pg::establish_connection_pg
};
use diesel::prelude::*;

use rocket::{form::Form, delete};
use rocket::{post, get};

use rocket::serde::json::{json, Value};
use uuid::{Builder, Uuid};
use rand::prelude::*;

#[get("/list")]
pub fn list() -> Option<Value> {
    let preferences = Preference::find_all();

    if &preferences.len() > &0 {
        let preference = &preferences[0];
        Some(json!(preference))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let preference = Preference::find_by_id(uuid);
    
    Some(json!(preference))
}

// #[get("/email/<email>")]
// pub fn find_by_email(email: &str) -> Option<Value> {
//     let preference = Preference::find_by_email(email);
    
// Some(json!(preference))
// }

#[get("/delete")]
pub fn delete_page() -> Option<Value> {
    let preferences = Preference::find_all();

    if &preferences.len() > &0 {
        let preference = &preferences[0];
        Some(json!(preference))
    } else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = Preference::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_preference: web::Json,
// ) -> Option<Value> {
//     let new_preference = Preference::update(id.into_inner(), new_preference.into_inner());
// }


#[post("/create", data = "<preference>")]
pub fn create(preference: Form<NewPreference>) -> Option<Value> {
    if preference.display_name != "" {
        let connection = &mut establish_connection_pg();
        
        let mut data = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut data);
    
        let uuid = Builder::from_random_bytes(data).into_uuid();
        
        let new_preference = Preference {
            id: uuid,
            browser_theme: preference.browser_theme.to_owned(),
            display_name: preference.display_name.to_owned(),
            pronouns: preference.pronouns.to_owned(),
        };

        diesel::insert_into(self::schema::preferences::dsl::preferences)
            .values(&new_preference)
            .execute(connection)
            .expect("Error saving new preference");

        Some(json!(new_preference))
    } else {
        None
    }
}
