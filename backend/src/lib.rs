use std::fmt::Display;

use form::{formvalue_to_survey, parse_markdown_text, Block, BlockType};
use wasm_bindgen::prelude::*;

use derive_builder::Builder;
use getrandom::getrandom;

use serde::{Deserialize, Serialize};

// use crate::form::parse_serialize_markdown_text;

mod form;

const NANOID_LEN: usize = 12;
// const NANOID_ALPHA: [char; 36] = [
//     '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
//     'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];
const NANOID_ALPHA: [char; 34] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NanoId(String);
impl NanoId {
    pub fn new() -> NanoId {
        NanoId(nanoid_gen(NANOID_LEN))
    }

    pub fn from(pre: &str) -> NanoId {
        return NanoId(format!("{}_{}", pre, nanoid_gen(NANOID_LEN)));
    }
    pub fn from_len(length: usize) -> NanoId {
        return NanoId(nanoid_gen(length));
    }
}

impl Display for NanoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// pub fn nanoid_gen() -> String {
//     let random =
//         [(); NANOID_LEN].map(|_| NANOID_ALPHA[thread_rng().gen_range(0..NANOID_ALPHA.len())]);
//     return String::from_iter(random.iter());
// }

// #[wasm_bindgen]
pub fn nanoid_gen(size: usize) -> String {
    let mask = NANOID_ALPHA.len().next_power_of_two() - 1;

    let mut res = String::new();
    let mut random: [u8; 32] = [0; 32];

    loop {
        getrandom(&mut random).unwrap();

        for &byte in random.iter() {
            let masked = byte as usize & mask;
            if masked < NANOID_ALPHA.len() {
                res.push(NANOID_ALPHA[masked]);
            }
            if res.len() == size {
                return res;
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserInfo {
    user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(default)]
pub struct Metadata {
    pub created_at: String,
    pub modified_at: String,
    pub version: String,
    pub id: String,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            created_at: chrono::offset::Utc::now().to_string(),
            modified_at: chrono::offset::Utc::now().to_string(),
            version: "0".to_string(),
            id: nanoid_gen(NANOID_LEN),
        }
    }
}

// #[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Survey {
    #[serde(flatten)]
    pub survey: ParsedSurvey,
    // pub id: String,
    // pub plaintext: String,
    // pub user_id: String,
    // pub created_at: String,
    // pub modified_at: String,
    // pub questions: Vec<Question>,
    // pub version: String,
    // pub parse_version: String,
    #[serde(flatten)]
    pub metadata: Metadata,
    // #[serde(flatten)]
    // pub UserInfo: UserInfo,
    pub user_id: Option<String>,
}

// #[wasm_bindgen]
impl Survey {
    // #[wasm_bindgen]
    pub fn from(parsed: ParsedSurvey) -> Survey {
        Survey {
            survey: parsed,
            metadata: Metadata::default(),
            user_id: None,
        }
    }

    pub fn new() -> Self {
        Self {
            survey: ParsedSurvey::new(),
            metadata: Metadata::default(),
            user_id: None,
        }
    }
}

impl Default for Survey {
    fn default() -> Self {
        Self::new()
    }
}

// #[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub id: String,
    pub value: String,
    pub options: Vec<QuestionOption>,
    pub r#type: QuestionType,
    pub created_on: String,
    pub modified_on: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct QuestionOption {
    pub id: String,
    pub text: String,
}

// #[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionType {
    Radio,
    Checkbox,
    Text,
    Number,
    Email,
    Date,
    Textarea,
    Submit,
}

#[wasm_bindgen]
pub fn markdown_to_form_wasm_v2(contents: String) -> JsValue {
    let survey = ParsedSurvey::from(contents);
    match survey {
        Ok(x) => serde_wasm_bindgen::to_value(&x).unwrap(),
        // This is a parsing issue, return something helpful to the user
        Err(err) => serde_wasm_bindgen::to_value(&err.to_string()).unwrap(),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParsedSurvey {
    pub id: String,
    pub title: String,
    pub plaintext: String,
    pub questions: Vec<Question>,
    pub blocks: Vec<Block>,
    pub parse_version: String,
    pub validation: (bool, Vec<String>),
}

impl ParsedSurvey {
    pub fn from(plaintext: String) -> anyhow::Result<ParsedSurvey> {
        let formvalues = parse_markdown_text(&plaintext);
        match formvalues {
            Ok(x) => {
                let survey = formvalue_to_survey(x);
                Ok(survey)
            }
            // This is a parsing issue, return something helpful to the user
            Err(err) => Err(err.into()),
        }
    }

    pub fn validate_form(&mut self) {
        let mut validation_errors = vec![];

        if self.blocks.len() == 0 {
            self.validation = (false, vec!["No questions found".to_string()]);
            return;
        }

        if self
            .blocks
            .first()
            .expect("should have a title")
            .get_type()
            .ne(&BlockType::Title)
        {
            validation_errors.push("Missing Title at beginning".to_string());
        }

        if self
            .blocks
            .last()
            .expect("should have a submit button")
            .get_type()
            .ne(&BlockType::Submit)
        {
            validation_errors.push("Missing Submit button at end".to_string());
        }

        self.validation = (
            if validation_errors.len() == 0 {
                true
            } else {
                false
            },
            validation_errors,
        );
    }

    // fn from_details(id: &str, title: &str, plaintext: &str, questions: Vec<Question>) -> Self {
    //     let title = title.replace("# ", "");
    //     ParsedSurvey {
    //         id: id.to_owned(),
    //         title: title.to_owned(),
    //         plaintext: plaintext.to_owned(),
    //         questions: questions,
    //         parse_version: "".to_string(),
    //         blocks: vec![],
    //     }
    // }
    pub fn new() -> Self {
        ParsedSurvey {
            title: "".to_owned(),
            plaintext: "".to_owned(),
            questions: vec![],
            parse_version: "".to_string(),
            id: "fakeid".to_string(),
            blocks: vec![],
            validation: (false, vec!["No questions found".to_string()]),
        }
    }
}

impl Default for ParsedSurvey {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{markdown_to_form_wasm_v2, ParsedSurvey, Question};

    #[test]
    fn test_wasm_parsed_markdown() {
        let input = include_str!("../test_forms/formexample-minimal.md");
        let results = ParsedSurvey::from(input.to_string());
        dbg!(results);
    }
}
