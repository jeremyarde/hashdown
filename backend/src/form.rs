use anyhow::anyhow;
use chrono::{DateTime, Utc};
use pest::error::{Error, LineColLocation};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use tracing::info;

use crate::{
    nanoid_gen, NanoId, ParsedSurvey, Question, QuestionOption, QuestionType, Survey, NANOID_LEN,
};

#[derive(Parser)]
#[grammar = "form.pest"]
struct FormParser;

pub fn parse_markdown_text(contents: &str) -> anyhow::Result<Vec<FormValue>, Error<Rule>> {
    // use pest::iterators::Pair;

    info!("Parsing: {:?}", contents);

    let formtext = match FormParser::parse(Rule::form, &contents) {
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    fn parse_value(pair: Pair<Rule>) -> FormValue {
        let rule = pair.as_rule();
        let val = pair.as_str();
        // println!("{:?}", rule);
        match pair.as_rule() {
            Rule::header => FormValue::Title {
                text: pair.into_inner().as_str().to_string(),
            },
            Rule::text_input => FormValue::TextInput {
                text: pair.into_inner().as_str().to_string(),
            },
            Rule::checkbox => FormValue::Checkbox {
                properties: pair.into_inner().map(parse_value).collect(),
            },
            // Rule::checkbox => FormValue::Checkbox {
            //     properties: pair.into_inner().map(parse_value).collect(),
            // },
            // Rule::checkbox => FormValue::Checkbox(pair.into_inner().map(parse_value).collect()),
            Rule::radio => FormValue::Radio {
                properties: pair.into_inner().map(parse_value).collect(),
            },
            Rule::dropdown => FormValue::Dropdown {
                properties: pair.into_inner().map(parse_value).collect(),
            },
            Rule::submit => FormValue::Submit {
                text: pair.as_str().to_string(),
            },
            // Rule::comment => todo!(),
            Rule::question_text => FormValue::QuestionText {
                text: pair.as_str().to_string(),
            },
            Rule::listitem => FormValue::ListItem {
                properties: pair.into_inner().map(parse_value).collect(),
            },
            Rule::unchecked => FormValue::CheckedStatus { value: false },
            Rule::checked => FormValue::CheckedStatus { value: true },
            Rule::inner_default_value => FormValue::DefaultValue {
                text: pair.as_str().to_string(),
            },
            Rule::EOI => FormValue::Nothing,
            Rule::textarea => FormValue::Textarea {
                text: pair.as_str().to_string(),
            },
            Rule::comment
            | Rule::SPACE
            | Rule::emptyline
            | Rule::form
            | Rule::block
            | Rule::default_value => FormValue::Nothing,
        }
    }

    let data = formtext
        .map(|pair| parse_value(pair))
        .collect::<Vec<FormValue>>();

    return Ok(data);
}

// impl fmt::Display for FormValue<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

fn form_value_to_survey_part(pair: &FormValue) -> SurveyPart {
    match pair {
        FormValue::Title { text } => SurveyPart::Title {
            title: text.clone(),
        },
        // FormValue::TextInput(x) => todo!(),
        // FormValue::Empty => todo!(),
        // FormValue::Nothing => todo!(),
        FormValue::Checkbox { properties } => {
            let question = match properties.get(0).unwrap() {
                FormValue::QuestionText { text } => text.clone(),
                _ => unreachable!(),
            };
            let options: Vec<CheckboxItem> = properties[1..]
                .iter()
                .map(|formvalue| match formvalue {
                    FormValue::ListItem { properties } => {
                        let checked = match properties.get(0).unwrap() {
                            FormValue::CheckedStatus { value } => value.clone(),
                            _ => unreachable!(),
                        };
                        let optiontext = match properties.get(1).unwrap() {
                            FormValue::QuestionText { text } => text.clone(),
                            _ => unreachable!(),
                        };

                        return CheckboxItem {
                            checked,
                            text: optiontext,
                        };
                    }
                    _ => {
                        unreachable!()
                    }
                })
                .collect();

            SurveyPart::Checkbox(CheckboxQuestion {
                options: options,
                question: question,
            })
        }
        FormValue::Radio { properties } => {
            let question = match properties.get(0).unwrap() {
                FormValue::QuestionText { text } => text.clone(),
                _ => unreachable!(),
            };
            let options = properties[1..]
                .iter()
                .map(|formvalue| match formvalue {
                    FormValue::ListItem { properties } => {
                        // let checked = match properties.get(0).unwrap() {
                        //     FormValue::CheckedStatus { value } => value.clone(),
                        //     _ => unreachable!(),
                        // };
                        let optiontext = match properties.get(0).unwrap() {
                            FormValue::QuestionText { text } => text.clone(),
                            _ => unreachable!(),
                        };
                        return optiontext;
                    }
                    _ => {
                        unreachable!()
                    }
                })
                .collect();

            SurveyPart::Radio(RadioQuestion {
                options: options,
                question: question,
            })
        }
        FormValue::TextInput { text } => SurveyPart::TextInput {
            question: text.clone(),
        },
        FormValue::Dropdown { properties } => {
            let question = match properties.get(0).unwrap() {
                FormValue::QuestionText { text } => text.clone(),
                _ => unreachable!(),
            };
            let options = properties[1..]
                .iter()
                .map(|formvalue| match formvalue {
                    FormValue::ListItem { properties } => {
                        let optiontext = match properties.get(0).unwrap() {
                            FormValue::QuestionText { text } => text.clone(),
                            _ => unreachable!(),
                        };
                        return optiontext;
                    }
                    _ => {
                        unreachable!()
                    }
                })
                .collect();
            return SurveyPart::Dropdown { question, options };
        }
        FormValue::Submit { text } => SurveyPart::Submit { text: text.clone() },
        FormValue::Textarea { text } => SurveyPart::Textarea {
            question: text.clone(),
        },
        // FormValue::DefaultValue { text } => todo!(), // _ => SurveyPart::Nothing,
        _ => SurveyPart::Nothing,
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RadioQuestion {
    question: String,
    options: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CheckboxQuestion {
    question: String,
    options: Vec<CheckboxItem>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
struct CheckboxItem {
    checked: bool,
    text: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
// #[serde(untagged)]
#[serde(tag = "type")]
pub enum SurveyPart {
    Title {
        title: String,
    },
    Radio(RadioQuestion),
    Checkbox(CheckboxQuestion),
    Dropdown {
        question: String,
        options: Vec<String>,
    },
    TextInput {
        question: String,
    },
    Textarea {
        question: String,
    },
    Nothing,
    Submit {
        text: String,
    },
}
impl SurveyPart {
    fn get_block_type(&self) -> BlockType {
        match self {
            SurveyPart::Title { title } => BlockType::Title,
            SurveyPart::Radio(_) => BlockType::Radio,
            SurveyPart::Checkbox(_) => BlockType::Checkbox,
            SurveyPart::Dropdown { question, options } => BlockType::Dropdown,
            SurveyPart::TextInput { question } => BlockType::TextInput,
            SurveyPart::Textarea { question } => BlockType::Textarea,
            SurveyPart::Nothing => BlockType::Empty,
            SurveyPart::Submit { text } => BlockType::Submit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FormText {
    title: FormValue,
    questions: Vec<FormValue>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SurveyV2 {
    questions: Vec<Question>,
    blocks: Vec<Block>,
}

impl SurveyV2 {
    pub fn from(values: Vec<FormValue>) -> SurveyV2 {
        let blocks = values
            .iter()
            .map(|formvalue| formvalue_to_block(formvalue))
            .collect();
        SurveyV2 {
            questions: vec![],
            blocks,
        }
    }
}

fn formvalue_to_block(formvalue: &FormValue) -> Block {
    let survey_part = form_value_to_survey_part(formvalue);
    let block_type = survey_part.get_block_type();
    return Block {
        id: NanoId::new(),
        index: 0.0,
        properties: survey_part,
        block_type,
    };
}

pub struct Metadata {
    id: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlockType {
    Title,
    Radio,
    ListItem,
    Checkbox,
    Dropdown,
    TextInput,
    Empty,
    Textarea,
    Submit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    id: NanoId,
    index: f32,
    // #[serde(flatten)]
    properties: SurveyPart,
    block_type: BlockType,
    // content: Vec<NanoId>,
    // parent: NanoId,
}

pub fn formvalue_to_survey(formvalues: Vec<FormValue>) -> ParsedSurvey {
    let mut survey = ParsedSurvey {
        id: nanoid_gen(NANOID_LEN),
        title: "".to_string(),
        plaintext: "not implemented".to_string(),
        questions: vec![],
        parse_version: "2".to_string(),
        blocks: vec![],
    };

    for formvalue in formvalues {
        survey.blocks.push(formvalue_to_block(&formvalue));
    }
    return survey;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(untagged)]
// #[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum FormValue {
    Title { text: String },
    TextInput { text: String },
    Nothing,
    Radio { properties: Vec<FormValue> },
    Dropdown { properties: Vec<FormValue> },
    ListItem { properties: Vec<FormValue> },
    QuestionText { text: String },
    Submit { text: String },
    Textarea { text: String },
    CheckedStatus { value: bool },
    DefaultValue { text: String },
    Checkbox { properties: Vec<FormValue> },
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::form::{formvalue_to_survey, parse_markdown_text, FormText, FormValue, SurveyV2};

    #[test]
    fn test_parse_minimal() {
        let res = parse_markdown_text(include_str!("../formexample-minimal.md"));
        // // let res = do_thing();
        println!("{:?}", &res);

        let serialized = formvalue_to_survey(res.unwrap());
        // let serialized = json!(res.unwrap());
        println!("{:#?}", serialized);
    }

    #[test]
    fn test_parse_all() {
        let res = parse_markdown_text(include_str!("../formexample.md"));
        // // let res = do_thing();
        println!("{:?}", &res);

        let serialized = formvalue_to_survey(res.unwrap());
        println!("{:#?}", serialized);

        println!("{:#}", json!(serialized));
    }
}
