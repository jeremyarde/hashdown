// use wasm_bindgen::prelude::*;

use derive_builder::Builder;
// use rand::{thread_rng, Rng};
// use nanoid::nanoid;
use getrandom::getrandom;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Error};
use std::{collections::hash_map::RandomState, option};
use tracing::{debug, info};

use std::hash::{BuildHasher, Hasher};

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

#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
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

impl Survey {
    fn from(parsed: ParsedSurvey) -> Survey {
        Survey {
            survey: parsed,
            metadata: Metadata::default(),
            user_id: None,
        }
    }
}

// impl Survey {
//     fn from(partial_survey: ParsedSurvey, user_id: String) -> Survey {
//         return Survey {
//             // plaintext: partial_survey.plaintext,
//             metadata: Metadata::new(),
//             user_id,
//             survey: partial_survey,
//         };
//     }

//     pub fn new(plaintext: String) -> Survey {
//         Survey {
//             survey: ParsedSurvey::from(plaintext),
//             metadata: Metadata::new(),
//             user_id: ,
//         }
//     }
// }

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
}

impl Question {
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
            qtype = QuestionType::Checkbox;
            question_text = question_text.clone().replace("[c]", "");
            // question_text = temp;
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

        // println!("parse: {question_text:?}");
        return (qtype, question_text.to_string());
    }
}

// #[wasm_bindgen]
#[derive(Clone, Debug)]
pub enum Types {
    checkbox,
    radio,
    text,
}

// #[wasm_bindgen]
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Questions {
    pub qs: Vec<Question>,
}

// #[wasm_bindgen]
// impl Questions {
//     fn new() -> Self {
//         Questions { qs: vec![] }
//     }
// }

#[derive(Debug)]
enum LineType {
    Question,
    Option,
    Nothing,
    Title,
}

// #[wasm_bindgen]
// pub fn markdown_to_form_wasm(contents: String) -> JsValue {
//     let survey = parse_markdown_v3(contents);

//     return serde_wasm_bindgen::to_value(&survey).unwrap();
// }

#[derive(Debug)]
enum ParseError {
    MultipleTitle(String),
}

pub fn parse_markdown_v3(contents: String) -> anyhow::Result<ParsedSurvey> {
    const VERSION: &str = "0";

    let plaintext = contents.clone();
    let mut questions = vec![];
    let mut curr_question_text: &str = "";
    let mut curr_options: Vec<&str> = vec![];
    let _in_question = false;
    let mut last_line_type: LineType = LineType::Nothing;
    let _question_num = 0;
    let mut title = "";
    let mut curr_line_type: LineType = LineType::Nothing;
    let mut curr_line: &str;

    for line in contents.lines() {
        // println!("Curr line: {line}");
        match (find_line_type(line), &last_line_type) {
            (LineType::Question, LineType::Question) => {
                curr_line_type = LineType::Question;
                // new question after question, push prev, clear old
                questions.push(Question::from(curr_question_text, curr_options.clone()));
                curr_question_text = line;
                curr_options.clear();
                last_line_type = LineType::Question;
            }
            (LineType::Question, LineType::Nothing) => {
                // new question, push prev, clear options
                curr_line_type = LineType::Question;
                curr_question_text = line;
                curr_options.clear();
                last_line_type = LineType::Question;
            }
            (LineType::Question, LineType::Option) => {
                curr_line_type = LineType::Question;
                // new question, push prev, clear options
                questions.push(Question::from(curr_question_text, curr_options.clone()));
                curr_options.clear();
                curr_question_text = line;
            }
            (LineType::Option, LineType::Question) => {
                // option for new question, clear options, push option
                curr_line_type = LineType::Option;
                curr_options.clear();
                curr_options.push(line);
                last_line_type = LineType::Option;
            }
            (LineType::Option, LineType::Option) => {
                curr_line_type = LineType::Option;
                // new option same question, push option
                curr_options.push(line);
                last_line_type = LineType::Option;
            }
            (LineType::Title, LineType::Nothing) => {
                curr_line_type = LineType::Title;
                title = line.clone();
                last_line_type = LineType::Title;
            }
            (LineType::Question, LineType::Title) => {
                // First question
                curr_line_type = LineType::Question;
                curr_question_text = line;
                curr_options.clear();
                last_line_type = LineType::Question;
            }
            (LineType::Title, _) => {
                curr_line_type = LineType::Title;
                return Err(anyhow!(
                    "Found multiple titles, remove one line that starts with `# `"
                ));
            }
            _ => {
                curr_line_type = LineType::Nothing;
                last_line_type = LineType::Nothing;
            }
        }
        println!("{curr_line_type:?}: {line:?}");
        debug!("{curr_line_type:?}: {line:?}");
    }

    // adding the last question
    questions.push(Question::from(curr_question_text, curr_options.clone()));

    // let value = Questions { qs: questions };

    // let newq = TestQ {
    //     text: "test".to_string(),
    // };

    let survey = ParsedSurvey::from_details(title, &plaintext, questions);

    return Ok(survey);
    // return JsValue::from(value);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParsedSurvey {
    pub title: String,
    pub plaintext: String,
    pub questions: Vec<Question>,
    pub parse_version: String,
}

impl ParsedSurvey {
    pub fn from(plaintext: String) -> anyhow::Result<ParsedSurvey> {
        return parse_markdown_v3(plaintext);
    }

    fn from_details(title: &str, plaintext: &str, questions: Vec<Question>) -> Self {
        let title = title.replace("# ", "");

        ParsedSurvey {
            title: title.to_owned(),
            plaintext: plaintext.to_owned(),
            questions: questions,
            parse_version: "".to_string(),
        }
    }
    pub fn new() -> Self {
        ParsedSurvey {
            title: "".to_owned(),
            plaintext: "".to_owned(),
            questions: vec![],
            parse_version: "".to_string(),
        }
    }
}

fn find_line_type(line: &str) -> LineType {
    let linetype: LineType;
    if line.starts_with("# ") {
        return LineType::Title;
    }

    if !line.starts_with(' ') && line.starts_with(|c: char| c.eq(&'-') || c.is_ascii_digit()) {
        linetype = LineType::Question
    } else if line.starts_with(" ")
        && line
            .trim_start()
            .starts_with(|c: char| c.eq(&'-') || c.is_ascii_digit())
    {
        linetype = LineType::Option
    } else {
        linetype = LineType::Nothing;
    }
    linetype
}

enum MarkdownElement {
    Heading,
    List,
    ListItem,
    Nothing,
}

#[cfg(test)]
mod tests {
    use crate::{parse_markdown_v3, Question};

    #[test]
    fn test() {
        let teststring = "1. this is a test\n - option 1\n - opt 2";

        let content = String::from(teststring);
        let result = parse_markdown_v3(content);
        print!("test result: {:?}\n", result);
    }

    #[test]
    fn test_v3() {
        let teststring = r#"
# This is the title

1. Question number 1
  1. option 1
  2. option 2
2. Question number 2
3. Question number 3
  1. q3 option 1
  2. q3 option 2
"#;

        let res = parse_markdown_v3(teststring.to_string()).unwrap();

        assert_eq!(&res.title, "This is the title");
        assert!(&res.questions.get(0).unwrap().value.eq("Question number 1"));
        assert_eq!(
            &res.questions.get(0).unwrap().options.get(0).unwrap().text,
            "option 1"
        );

        assert_eq!(&res.questions.get(1).unwrap().value, "Question number 2");
        assert_eq!(&res.questions.get(1).unwrap().options.len(), &(0 as usize));

        assert_eq!(&res.questions.get(2).unwrap().value, "Question number 3");
        assert_eq!(
            &res.questions.get(2).unwrap().options.get(0).unwrap().text,
            "q3 option 1"
        );
        assert_eq!(
            &res.questions.get(2).unwrap().options.get(1).unwrap().text,
            "q3 option 2"
        );

        // println!("{:#?}", res)
    }

    #[test]
    fn test_question_parsing() {
        assert_eq!(
            Question::parse_question_type_and_text("- testing").1,
            "testing"
        );
        assert_eq!(
            Question::parse_question_type_and_text(" - testing").1,
            "testing"
        );
        assert_eq!(
            Question::parse_question_type_and_text("  - testing").1,
            "testing"
        );

        assert_eq!(
            Question::parse_question_type_and_text("1. testing").1,
            "testing"
        );
        assert_eq!(
            Question::parse_question_type_and_text(" 1. testing").1,
            "testing"
        );
        assert_eq!(
            Question::parse_question_type_and_text("  1. testing").1,
            "testing"
        );
    }
}
