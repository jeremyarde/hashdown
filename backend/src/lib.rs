use chrono::Utc;
use form::{formvalue_to_survey, parse_markdown_text, Block, FormValue, SurveyPart};
use wasm_bindgen::prelude::*;

use derive_builder::Builder;
// use rand::{thread_rng, Rng};
// use nanoid::nanoid;
use getrandom::getrandom;
use regex::Regex;
use serde::{Deserialize, Serialize};

use anyhow::anyhow;
use std::collections::hash_map::RandomState;
use tracing::debug;

use std::hash::{BuildHasher, Hasher};

// use crate::form::parse_serialize_markdown_text;

mod form;

fn rand64() -> u64 {
    RandomState::new().build_hasher().finish()
}

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
    fn new() -> NanoId {
        return NanoId(nanoid_gen(NANOID_LEN));
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

impl UserInfo {
    fn new(user_id: String) -> Self {
        return UserInfo { user_id: user_id };
    }
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

impl Question {
    fn from_v4(q_text: String, options: Vec<&str>, q_type: QuestionType) -> Question {
        Question {
            id: nanoid_gen(NANOID_LEN),
            value: q_text,
            options: options
                .iter()
                .map(|&x| QuestionOption {
                    id: nanoid_gen(NANOID_LEN),
                    text: x.to_owned(),
                })
                .collect(),
            r#type: q_type,
            created_on: Utc::now().to_string(),
            modified_on: Utc::now().to_string(),
        }
    }

    fn from(q_text: &str, options: Vec<&str>) -> Self {
        let question_id = nanoid_gen(NANOID_LEN);
        let (question_type, question_text) = Question::parse_question_type_and_text(q_text);
        return Question {
            // id: nanoid!(NANOID_LEN, &NANOID_ALPHA, random),
            id: question_id,
            value: question_text.clone(),
            options: options
                .iter()
                .map(|&option_value| {
                    let remove_start =
                        Regex::new(r"((?P<number>\d{1,}).|(?P<dash>-))(?P<content>\s.*)$").unwrap();

                    let mut clean = remove_start.replace(option_value, "$content").to_string();

                    // remove_start.replace(option_value);
                    clean = clean.trim().to_owned();
                    // clean.trim_end_matches(char::is_digit);
                    // clean.trim_start_matches(&[" -", "1. "]);
                    // todo!("Add regex here to remove either ' - ' from start or ' 1. ' ");

                    QuestionOption {
                        // id: "nanoid_gen()".to_string(),
                        id: nanoid_gen(12),
                        text: clean.to_string().to_owned(),
                    }
                })
                .collect(),
            r#type: question_type,
            created_on: "now".to_string(),
            modified_on: "now".to_string(),
        };
    }

    fn parse_question_type_and_text(line: &str) -> (QuestionType, String) {
        let qtype: QuestionType;
        let mut question_text = line.to_owned();
        // if line.contains("[checkbox]") || line.contains("[c]") {
        if question_text.contains("[c]") {
            // TODO: removed checkbox from supported answers
            // qtype = QuestionType::Checkbox;
            qtype = QuestionType::Radio;
            question_text = question_text.clone().replace("[c]", "");
            // question_text = temp;
        } else if question_text.contains("[t]") {
            qtype = QuestionType::Text;
            question_text = question_text.clone().replace("[t]", "");
        } else if question_text.contains("[n]") {
            qtype = QuestionType::Number;
            question_text = question_text.clone().replace("[n]", "");
        } else {
            qtype = QuestionType::Radio;
        }

        question_text = match question_text.trim_start().split_once("- ") {
            Some(x) => x.1.to_owned(),
            None => question_text,
        };

        let trimmed = question_text.trim_start();
        if trimmed.starts_with(char::is_numeric) {
            question_text = trimmed
                .split_once(". ")
                .unwrap_or((&question_text, ""))
                .1
                .to_owned();
        }

        return (qtype, question_text.to_string());
    }
}

#[wasm_bindgen]
pub fn markdown_to_form_wasm_v2(contents: String) -> JsValue {
    let survey = ParsedSurvey::from(contents);
    match survey {
        Ok(x) => {
            return serde_wasm_bindgen::to_value(&x).unwrap();
        }
        // This is a parsing issue, return something helpful to the user
        Err(err) => return serde_wasm_bindgen::to_value(&err.to_string()).unwrap(),
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
}

impl ParsedSurvey {
    pub fn from(plaintext: String) -> anyhow::Result<ParsedSurvey> {
        let formvalues = parse_markdown_text(&plaintext);
        match formvalues {
            Ok(x) => {
                let survey = formvalue_to_survey(x);
                return Ok(survey);
            }
            // This is a parsing issue, return something helpful to the user
            Err(err) => return Err(err.into()),
        }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{markdown_to_form_wasm_v2, ParsedSurvey, Question};

    #[test]
    fn test_wasm_parsed_markdown() {
        let input = include_str!("../formexample-minimal.md");
        let results = ParsedSurvey::from(input.to_string());
        dbg!(results);
    }
}
