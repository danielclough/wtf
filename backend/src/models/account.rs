use crate::{
    schema::accounts,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::{request::{self, Request}, serde::json::serde_json};
use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::http::{Status, ContentType};
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use super::_common::Error;

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = accounts)]
#[diesel(belongs_to(Preference))]
#[diesel(belongs_to(Relationship))]
#[diesel(belongs_to(SurveyResults))]
#[diesel(belongs_to(User))]
pub struct NewAccount<'r> {
    pub avatar:  &'r str,
    pub level:  &'r str,
    pub preference_ids: Vec<Option<String>>,
    pub relationship_ids: Vec<Option<String>>,
    pub survey_results_ids: Vec<Option<String>>,
    pub user_ids: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(belongs_to(Preference))]
#[diesel(belongs_to(Relationship))]
#[diesel(belongs_to(SurveyResults))]
#[diesel(belongs_to(User))]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: Uuid,
    pub avatar: String,
    pub level: String,
    pub preference_ids: Vec<Option<Uuid>>,
    pub relationship_ids: Vec<Option<Uuid>>,
    pub survey_results_ids: Vec<Option<Uuid>>,
    pub user_ids: Vec<Option<Uuid>>,
}

impl Account {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let accounts = accounts::table.load::<Account>(conn).expect("db connection");
        accounts
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let account = accounts::table.filter(accounts::id.eq(id)).first(conn).expect("db connection");
        account
    }
    pub fn find_by_user(user_id: &str) -> Self {
        let uuid = Uuid::parse_str(user_id).expect("uuid");
        let conn = &mut establish_connection_pg();
        let account = accounts::table.filter(accounts::user_ids.is_contained_by(vec![uuid])).first(conn).expect("db connection");
        account
    }
    pub fn create(account: NewAccount) -> Self {
        let conn = &mut establish_connection_pg();
        let account = NewAccount::from(account);
        let account = diesel::insert_into(accounts::table)
            .values(account)
            .get_result(conn).expect("db connection");
        account
    }
    pub fn update(id: Uuid, account: Account) -> Self {
        let conn = &mut establish_connection_pg();
        let account = diesel::update(accounts::table)
            .filter(accounts::id.eq(id))
            .set(account)
            .get_result(conn).expect("db connection");
        account
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(accounts::table.filter(accounts::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewAccount<'_> {
    fn from(account: NewAccount) -> Account {
        let uuid = new_random_uuid_v4();
        Account {
            id: uuid,
            avatar: account.avatar.to_string(),
            level: account.level.to_string(),
            preference_ids: Some(account.preference_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect()).get_or_insert(vec![Some(uuid)]).to_vec(),
            relationship_ids: Some(account.relationship_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect()).get_or_insert(vec![Some(uuid)]).to_vec(),
            survey_results_ids: Some(account.survey_results_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect()).get_or_insert(vec![Some(uuid)]).to_vec(),
            user_ids: Some(account.user_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect()).get_or_insert(vec![Some(uuid)]).to_vec(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewAccount<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_account_ct = ContentType::new("application", "x-new_account");
        if req.content_type() != Some(&new_account_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_account' or fallback to default.
        let limit = req.limits().get("new_account").unwrap_or(1.mebibytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewAccount = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}