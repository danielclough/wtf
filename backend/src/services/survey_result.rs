use crate::{
    models::survey_result::{NewSurveyResult, SurveyResult}
};
use rocket::serde::json::{json, Value};

use rocket::{form::Form, delete};
use rocket::{post, get};

use uuid::Uuid;

#[post("/create", data = "<survey_result>")]
pub fn create(survey_result: Form<NewSurveyResult>) -> Option<Value> {
    if true {
        
        let new_survey_result = NewSurveyResult {
            aesthetics: survey_result.aesthetics.to_owned(),
            cognitive: survey_result.cognitive.to_owned(),
            cosmology: survey_result.cosmology.to_owned(),
            environmental: survey_result.environmental.to_owned(),
            epistemology: survey_result.epistemology.to_owned(),
            ethics: survey_result.ethics.to_owned(),
            history: survey_result.history.to_owned(),
            isms: survey_result.isms.to_owned(),
            law: survey_result.law.to_owned(),
            logic: survey_result.logic.to_owned(),
            maths: survey_result.maths.to_owned(),
            ontology: survey_result.ontology.to_owned(),
            political: survey_result.political.to_owned(),
            rhetoric: survey_result.rhetoric.to_owned(),
            science: survey_result.science.to_owned(),
            theology: survey_result.theology.to_owned(),
        };

        let survey_result = SurveyResult::create(new_survey_result);

        Some(json!(survey_result))
    } else {
        None
    }
}

#[get("/list")]
pub fn list() -> Option<Value> {
    let survey_results = SurveyResult::find_all();

    if &survey_results.len() > &0 {
        Some(json!(&survey_results[0]))
    } else {
        None
    }
}


#[get("/<id>")]
pub fn find_by_id(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    let survey_result = SurveyResult::find_by_id(uuid);
    
    if true {
        Some(json!(survey_result))
    } else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: &str) -> Option<Value> {
    let uuid = Uuid::parse_str(id).expect("parse uuid");
    _ = SurveyResult::delete(uuid);
    
    None
}

// #[put("/<id>")]
// async fn update(
//     id: web::Path,
//     new_survey_result: web::Json,
// ) -> Option<Value> {
//     let new_survey_result = SurveyResult::update(id.into_inner(), new_survey_result.into_inner());
// }
