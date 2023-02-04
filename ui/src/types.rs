use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SurveyDto {
    pub id: String,
    pub nanoid: String,
    pub plaintext: String,
    pub user_id: String,
    pub created_at: String,
    pub modified_at: String,
    pub version: String,
}
