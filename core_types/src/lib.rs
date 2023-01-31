// use ormlite::Model;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateSurvey {
    pub id: String,
    pub plaintext: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListSurvey {
    pub surveys: Vec<Survey>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Survey {
    id: i32,
    nanoid: String,
    plaintext: String,
    user_id: String,
    created_at: String,
    modified_at: String,
    version: String,
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
