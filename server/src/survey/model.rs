#[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
pub struct Survey {
    pub id: String,
    // nanoid: String,
    pub plaintext: String,
    // user_id: String,
    // created_at: String,
    // modified_at: String,
    // version: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListSurveyResponse {
    pub surveys: Vec<Survey>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyRequest {
    pub plaintext: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyResponse {
    pub survey: Survey,
}
