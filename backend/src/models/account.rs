use crate::{
    schema::accounts,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, FromForm, Debug)]
pub struct NewAccount {
    pub avatar: String,
    pub level: String,
    pub preference_id: String,
    pub role_ids: Vec<Option<String>>,
    pub sensitivity_ids: Vec<Option<String>>,
    pub survey_results_id: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = accounts)]
#[diesel(belongs_to(SurveyResults))]
#[diesel(belongs_to(Role))]
#[diesel(belongs_to(Event))]
#[diesel(belongs_to(Preference))]
pub struct Account {
    pub id: Uuid,
    pub avatar: String,
    pub level: String,
    pub preference_id: Uuid,
    pub role_ids: Vec<Option<Uuid>>,
    pub sensitivity_ids: Vec<Option<Uuid>>,
    pub survey_results_id: Uuid,
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
impl NewAccount {
    fn from(account: NewAccount) -> Account {
        let uuid = new_random_uuid_v4();
        Account {
            id: uuid,
            avatar: account.avatar,
            level: account.level,
            preference_id: Uuid::parse_str(&account.preference_id).expect("uuid"),
            role_ids: account.role_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            sensitivity_ids: account.sensitivity_ids.iter().map(|x| Some(Uuid::parse_str(&x.clone().expect("some")).expect("uuid"))).collect(),
            survey_results_id: Uuid::parse_str(&account.survey_results_id).expect("uuid"),
        }
    }
}
