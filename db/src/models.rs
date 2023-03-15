// use ormlite::Model;
use rand::{thread_rng, Rng};
use serde::{self, Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
const NANOID_LEN: usize = 12;

// const NANOID_ALPHA: [char; 36] = [
//     '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
//     'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];
const NANOID_ALPHA: [char; 34] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub fn nanoid_gen() -> String {
    let random =
        [(); NANOID_LEN].map(|_| NANOID_ALPHA[thread_rng().gen_range(0..NANOID_ALPHA.len())]);
    return String::from_iter(random.iter());
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnswersModel {
    id: String,
    external_id: String,
    survey_id: String,
    survey_version: String,
    start_time: String,
    end_time: String,
    answers: String,
    created_at: String,
}

impl CreateAnswersModel {
    pub fn from(create_answer: CreateAnswersRequest) -> CreateAnswersModel {
        CreateAnswersModel {
            id: nanoid_gen(),
            // id: "test".to_string(),
            external_id: nanoid_gen(),
            survey_id: create_answer.survey_id,
            survey_version: create_answer.survey_version,
            start_time: create_answer.start_time,
            end_time: "now".to_string(),
            answers: json!(create_answer.answers).to_string(),
            created_at: "now".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnswersRequest {
    id: String,
    pub survey_id: String,
    pub survey_version: String,
    pub start_time: String,
    pub answers: HashMap<String, AnswerDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnswerDetails {
    pub values: Vec<String>,
    pub r#type: AnswerType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AnswerType {
    Float,
    String,
    Integer,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAnswersResponse {
    id: String,
    survey_id: String,
    survey_version: String,
    start_time: String,
    answers: HashMap<String, AnswerDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyRequest {
    plaintext: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyResponse {
    survey: Survey,
}

#[cfg(test)]
mod tests {
    use crate::models::nanoid_gen;

    #[test]
    fn testing_random() {
        let res = nanoid_gen();
        println!("{res}");
    }
}
