use wasm_bindgen::prelude::*;



// use nanoid::nanoid;
use getrandom::getrandom;
use regex::Regex;
use serde::{Deserialize, Serialize};




use std::collections::hash_map::RandomState;
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

#[wasm_bindgen]
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
pub struct Survey {
    pub id: String,
    pub plaintext: String,
    pub user_id: String,
    pub created_at: String,
    pub modified_at: String,
    pub questions: Vec<Question>,
    pub version: String,
    pub parse_version: String,
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

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum QuestionType {
    Radio,
    Checkbox,
    Text,
}

impl Question {
    fn from(q_text: &str, options: Vec<&str>) -> Self {
        return Question {
            // id: nanoid!(NANOID_LEN, &NANOID_ALPHA, random),
            id: nanoid_gen(NANOID_LEN),
            value: Question::parse_question_text(q_text).to_string(),
            options: options
                .iter()
                .map(|&o| QuestionOption {
                    id: nanoid_gen(NANOID_LEN),
                    text: Question::parse_question_text(o).to_string(),
                })
                .collect(),
            r#type: Question::parse_question_type(q_text),
            created_on: "now".to_string(),
            modified_on: "now".to_string(),
        };
    }

    fn parse_question_type(line: &str) -> QuestionType {
        let res: QuestionType;
        if line.contains("[checkbox]") || line.contains("[c]]") {
            res = QuestionType::Checkbox;
        } else {
            res = QuestionType::Radio;
        }

        res
    }

    fn parse_question_text(line: &str) -> &str {
        let trimmed = line.clone().trim_start();
        let mut question_text = match line.trim_start().split_once("- ") {
            Some(x) => x.1,
            None => line,
        };

        if trimmed.starts_with(char::is_numeric) {
            question_text = trimmed.split_once(". ").unwrap_or((line, "")).1;
        }

        // println!("parse: {question_text:?}");
        return question_text;
    }

    // pub async fn insert(
    //     &mut self,
    //     survey: PutSurveyRequest,
    //     pool: SqlitePool,
    // ) -> anyhow::Result<()> {
    //     println!("To insert: {:?}", survey);

    //     let res = sqlx::query("Insert into surveys (plaintext) values ($1) returning *")
    //         .bind(survey.plaintext)
    //         .execute(&mut pool.acquire().await?)
    //         .await?;
    //     // let mut query_builder: QueryBuilder<Sqlite> =
    //     //     QueryBuilder::new(TodoModel::create_insert_sql());
    //     // query_builder.push_values(todos.into_iter().take(512), |mut b, x| {
    //     //     info!("todo to be entered: {x:?}");
    //     //     b.push_bind(x.id)
    //     //         .push_bind(x.status)
    //     //         .push_bind(x.description)
    //     //         .push_bind(x.file)
    //     //         .push_bind(x.last_updated)
    //     //         .push_bind(x.last_indexed)
    //     //         .push_bind(x.due);
    //     // });
    //     // let res = query_builder.build().execute(&mut self.pool).await?;

    //     info!("database insert results; #={:?}", res);

    //     // potentially one way to make sure we don't overwrite certain fields:
    //     // https://stackoverflow.com/questions/3634984/insert-if-not-exists-else-update
    //     Ok(())
    // }
}

#[derive(Clone, Debug)]
struct Answer {
    survey_id: i32,
    name: String,
    question_number: i32,
    answer: String,
}

#[wasm_bindgen]
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

#[wasm_bindgen]
impl Questions {
    fn new() -> Self {
        Questions { qs: vec![] }
    }
}

pub fn parse_markdown_blocks(markdown: String) -> Questions {
    // let markdown = include_str!("../test_file.md").to_string();
    let questions = Regex::new(r"(?m)^(\d). (.*)$").unwrap();
    let _locations = questions.captures_iter(&markdown);
    // for x in locations {
    //     println!("{:#?}", x);
    // }
    let mut questions = vec![];

    let mut current_question: &str;
    let mut options: Vec<&str> = vec![];
    let mut question_id = 1;

    let mut current = 1;
    // for line in markdown.lines() {
    let mut lines = markdown.lines();
    let mut currline = match lines.next() {
        Some(x) => x,
        None => return Questions { qs: vec![] },
    };

    loop {
        let q_num = format!("{}. ", current);
        println!("{}", currline);
        // Is a question
        if currline.starts_with(q_num.as_str()) {
            current += 1;

            // current_question = currline.trim_start_matches(q_num.as_str()).to_owned();
            current_question = currline;
            // current_question = parse_question_text(currline).to_owned();

            currline = match lines.next() {
                Some(x) => x,
                None => {
                    println!("Did not find a new line to parse");
                    continue;
                }
            };
            println!("{}", currline);
            while currline.starts_with("  ") {
                options.push(currline);
                currline = match lines.next() {
                    Some(x) => x,
                    None => break,
                };
            }

            questions.push(Question::from(&current_question, options));
            options = vec![];
            question_id += 1;
        } else {
            println!("next: {}", currline);
            currline = match lines.next() {
                Some(x) => x,
                None => break,
            };
        }
    }

    println!("{:#?}", questions);
    Questions { qs: questions }
}

enum LineType {
    Question,
    Option,
    Nothing,
}

pub fn markdown_to_form(contents: String) -> Survey {
    let survey = parse_markdown_v3(contents);
    return survey;
}

#[wasm_bindgen]
pub fn markdown_to_form_wasm(contents: String) -> JsValue {
    let survey = parse_markdown_v3(contents);

    return serde_wasm_bindgen::to_value(&survey).unwrap();
}

pub fn parse_markdown_v3(contents: String) -> Survey {
    // let mut questions = Questions::new();
    let mut questions = vec![];
    let mut curr_question_text: &str = "";
    let mut curr_options: Vec<&str> = vec![];
    let _in_question = false;
    let mut last_line_type: LineType = LineType::Nothing;
    let _question_num = 0;

    for line in contents.lines() {
        // println!("Curr line: {line}");
        match (find_line_type(line), &last_line_type) {
            (LineType::Question, LineType::Question) => {
                // new question after question, push prev, clear old
                questions.push(Question::from(curr_question_text, curr_options.clone()));
                curr_question_text = line;
                curr_options.clear();
                last_line_type = LineType::Question;
            }
            (LineType::Question, LineType::Nothing) => {
                // new question, push prev, clear options
                // questions.push(Question::from(curr_question_text, curr_options.clone()));
                curr_question_text = line;
                curr_options.clear();
                last_line_type = LineType::Question;
            }
            (LineType::Question, LineType::Option) => {
                // new question, push prev, clear options
                questions.push(Question::from(curr_question_text, curr_options.clone()));
                curr_options.clear();
                curr_question_text = line;
            }
            (LineType::Option, LineType::Question) => {
                // option for new question, clear options, push option
                curr_options.clear();
                curr_options.push(line);
                last_line_type = LineType::Option;
            }
            (LineType::Option, LineType::Option) => {
                // new option same question, push option
                curr_options.push(line);
                last_line_type = LineType::Option;
            }
            _ => {}
        }
    }

    // adding the last question
    questions.push(Question::from(curr_question_text, curr_options.clone()));

    // let value = Questions { qs: questions };

    // let newq = TestQ {
    //     text: "test".to_string(),
    // };

    let survey = Survey {
        id: nanoid_gen(NANOID_LEN),
        plaintext: contents,
        user_id: "".to_string(),
        created_at: "".to_string(),
        modified_at: "".to_string(),
        questions: questions,
        version: "0".to_string(),
        parse_version: "0".to_string(),
    };
    return survey;
    // return JsValue::from(value);
}

fn find_line_type(line: &str) -> LineType {
    let linetype: LineType;
    if !line.starts_with(" ") && line.starts_with(|c: char| c.eq(&'-') || c.is_digit(10)) {
        linetype = LineType::Question
    } else if line.starts_with(" ")
        && line
            .trim_start()
            .starts_with(|c: char| c.eq(&'-') || c.is_digit(10))
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
    use crate::{nanoid_gen, parse_markdown_blocks, parse_markdown_v3, Question};

    #[test]
    fn test() {
        let teststring = "1. this is a test\n  ";

        let content = String::from(teststring);
        let result = parse_markdown_blocks(content);
        print!("test result: {:?}\n", result);
    }

    #[test]
    fn test_v3() {
        let teststring = r#"
1. Question number 1
  1. option 1
  2. option 2
2. Question number 2
3. Question number 3
  1. q3 option 1
  2. q3 option 2
"#;

        let res = parse_markdown_v3(teststring.to_string());

        println!("{:#?}", res)
    }

    #[test]
    fn test_bullet_points() {
        let teststring = r#"
- Question number 1
  - option 1
  - option 2
- Question number 2
- Question number 3
  - q3 option 1
  - q3 option 2
"#;

        let res = parse_markdown_v3(teststring.to_string());

        println!("{:#?}", res)
    }

    #[test]
    fn test_nanoid_gen() {
        let nanoid = nanoid_gen(10);
        println!("nanoid: {nanoid:?}");
    }

    #[test]
    fn test_question_parsing() {
        assert_eq!(Question::parse_question_text("- testing"), "testing");
        assert_eq!(Question::parse_question_text(" - testing"), "testing");
        assert_eq!(Question::parse_question_text("  - testing"), "testing");

        assert_eq!(Question::parse_question_text("1. testing"), "testing");
        assert_eq!(Question::parse_question_text(" 1. testing"), "testing");
        assert_eq!(Question::parse_question_text("  1. testing"), "testing");
    }
}
