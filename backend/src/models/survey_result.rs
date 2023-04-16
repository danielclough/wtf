use crate::{
    schema::survey_results,
    utils::{
        pg::establish_connection_pg, uuid::new_random_uuid_v4
    }
};
use rocket::FromForm;
use diesel::{prelude::*};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Queryable, Insertable, Serialize, Deserialize, AsChangeset, FromForm, Debug)]
#[diesel(table_name = survey_results)]
pub struct NewSurveyResult {
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
    pub fn update(id: Uuid, survey_results: SurveyResult) -> Self {
        let conn = &mut establish_connection_pg();
        let survey_results = diesel::update(survey_results::table)
            .filter(survey_results::id.eq(id))
            .set(survey_results)
            .get_result(conn).expect("db connection");
        survey_results
    }
    pub fn delete(id: Uuid) -> usize {
        let conn = &mut establish_connection_pg();
        let res = diesel::delete(survey_results::table.filter(survey_results::id.eq(id))).execute(conn).expect("db connection");
        res
    }
}
impl NewSurveyResult {
    fn from(survey_results: NewSurveyResult) -> SurveyResult {
        let uuid = new_random_uuid_v4();
        SurveyResult {
            id: uuid,
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
