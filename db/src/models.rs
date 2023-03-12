use serde::{self, Deserialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateAnswersModel {
    id: String,
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
            id: nanoid_gen(12),
            survey_id: create_answer.survey_id,
            survey_version: create_answer.survey_version,
            start_time: create_answer.start_time,
            end_time: "now".to_string(),
            answers: json!(create_answer.answers).to_string(),
            created_at: "now".to_string(),
        }
    }
}
