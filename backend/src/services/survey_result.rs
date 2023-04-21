use crate::{
    models::survey_result::{NewSurveyResult, SurveyResult}
};
use rocket::serde::json::{json, Value};

use rocket::{form::Form, delete};
use rocket::{post, get};

use uuid::Uuid;

#[post("/create", data = "<body>")]
pub fn create(body: NewSurveyResult<'_>) -> Option<Value> {
    if true {
        let new_survey_result = body;

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
