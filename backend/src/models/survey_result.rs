use crate::{
    schema::survey_results,
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
#[diesel(table_name = survey_results)]
pub struct NewSurveyResult<'r> {
    pub timestamp:  &'r str,
    pub aesthetics: Vec<Option<String>>,
    pub cognitive: Vec<Option<String>>,
    pub cosmology: Vec<Option<String>>,
    pub environmental: Vec<Option<String>>,
    pub epistemology: Vec<Option<String>>,
    pub ethics: Vec<Option<String>>,
    pub history: Vec<Option<String>>,
    pub isms: Vec<Option<String>>,
    pub law: Vec<Option<String>>,
    pub logic: Vec<Option<String>>,
    pub maths: Vec<Option<String>>,
    pub ontology: Vec<Option<String>>,
    pub political: Vec<Option<String>>,
    pub rhetoric: Vec<Option<String>>,
    pub science: Vec<Option<String>>,
    pub theology: Vec<Option<String>>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, Debug)]
// #[diesel(belongs_to(Account))]
#[diesel(table_name = survey_results)]
pub struct SurveyResult {
    pub id: Uuid,
    pub timestamp: String,
    pub aesthetics: Vec<Option<String>>,
    pub cognitive: Vec<Option<String>>,
    pub cosmology: Vec<Option<String>>,
    pub environmental: Vec<Option<String>>,
    pub epistemology: Vec<Option<String>>,
    pub ethics: Vec<Option<String>>,
    pub history: Vec<Option<String>>,
    pub isms: Vec<Option<String>>,
    pub law: Vec<Option<String>>,
    pub logic: Vec<Option<String>>,
    pub maths: Vec<Option<String>>,
    pub ontology: Vec<Option<String>>,
    pub political: Vec<Option<String>>,
    pub rhetoric: Vec<Option<String>>,
    pub science: Vec<Option<String>>,
    pub theology: Vec<Option<String>>,
}

impl SurveyResult {
    pub fn find_all() -> Vec<Self> {
        let conn = &mut establish_connection_pg();
        let survey_results = survey_results::table.load::<SurveyResult>(conn).expect("db connection");
        survey_results
    }
    pub fn find_by_id(id: Uuid) -> Self {
        let conn = &mut establish_connection_pg();
        let survey_results = survey_results::table.filter(survey_results::id.eq(id)).first(conn).expect("db connection");
        survey_results
    }
    pub fn create(survey_results: NewSurveyResult) -> Self {
        let conn = &mut establish_connection_pg();
        let survey_results = NewSurveyResult::from(survey_results);
        let survey_results = diesel::insert_into(survey_results::table)
            .values(survey_results)
            .get_result(conn).expect("db connection");
        survey_results
    }
    pub fn update(id: Uuid, new_survey_result: NewSurveyResult) -> Self {
        let conn = &mut establish_connection_pg();

        let updated = NewSurveyResult::from_existing(id, new_survey_result);

        let survey_results = diesel::update(survey_results::table)
            .filter(survey_results::id.eq(id))
            .set(updated)
            .get_result(conn).expect("db connection");
        survey_results
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(survey_results::table.filter(survey_results::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewSurveyResult<'_> {
    fn from(survey_results: NewSurveyResult) -> SurveyResult {
        let uuid = new_random_uuid_v4();
        SurveyResult {
            id: uuid,
            timestamp: survey_results.timestamp.to_string(),
            aesthetics: survey_results.aesthetics,
            cognitive: survey_results.cognitive,
            cosmology: survey_results.cosmology,
            environmental: survey_results.environmental,
            epistemology: survey_results.epistemology,
            ethics: survey_results.ethics,
            history: survey_results.history,
            isms: survey_results.isms,
            law: survey_results.law,
            logic: survey_results.logic,
            maths: survey_results.maths,
            ontology: survey_results.ontology,
            political: survey_results.political,
            rhetoric: survey_results.rhetoric,
            science: survey_results.science,
            theology: survey_results.theology,
        }
    }
    fn from_existing(id: Uuid, survey_results: NewSurveyResult) -> SurveyResult {
        SurveyResult {
            id,
            timestamp: survey_results.timestamp.to_string(),
            aesthetics: survey_results.aesthetics,
            cognitive: survey_results.cognitive,
            cosmology: survey_results.cosmology,
            environmental: survey_results.environmental,
            epistemology: survey_results.epistemology,
            ethics: survey_results.ethics,
            history: survey_results.history,
            isms: survey_results.isms,
            law: survey_results.law,
            logic: survey_results.logic,
            maths: survey_results.maths,
            ontology: survey_results.ontology,
            political: survey_results.political,
            rhetoric: survey_results.rhetoric,
            science: survey_results.science,
            theology: survey_results.theology,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for NewSurveyResult<'r> {
    type Error = Error;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use Error::*;
        use rocket::outcome::Outcome::*;

        // Ensure the content type is correct before opening the data.
        let new_survey_results_ct = ContentType::new("application", "x-new_survey_result");
        if req.content_type() != Some(&new_survey_results_ct) {
            return Forward(data);
        }

        // Use a configured limit with name 'new_survey_results' or fallback to default.
        let limit = req.limits().get("new_survey_result").unwrap_or(1.mebibytes());

        // Read the data into a string.
        let string = match data.open(limit).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return Failure((Status::PayloadTooLarge, TooLarge)),
            Err(e) => return Failure((Status::InternalServerError, Io(e))),
        };

        // We store `string` in request-local cache for long-lived borrows.
        let string = request::local_cache!(req, string);
        println!("{}", string);

        let data: NewSurveyResult = serde_json::from_str(string).expect("works");
        
        Success(data)
    }
}