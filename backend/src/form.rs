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

// #[derive(Debug, Serialize, Deserialize)]
// pub enum QuestionType {
//     Checkbox,
// }

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
            Rule::textarea => FormValue::TextArea {
                text: pair.as_str().to_string(),
            },
            Rule::comment
            | Rule::SPACE
            | Rule::emptyline
            | Rule::form
            | Rule::block
            | Rule::default_value => FormValue::Empty,
        }
    }

    let data = formtext
        .map(|pair| parse_value(pair))
        .collect::<Vec<FormValue>>();

    // let survey: SurveyV2 = SurveyV2::from(data);

    return Ok(data);
}

// impl fmt::Display for FormValue<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

fn form_value_to_survey_part(pair: &FormValue) -> SurveyPart {
    match pair {
        FormValue::Title { text } => SurveyPart::Title(text.to_owned()),
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
        // get_bool(&x[0]),       // x[1..].iter().map(|x| serialize_form_value(x)).collect(),
        // FormValue::Radio(x) => {
        //     return SurveyPart::Radio(
        //         get_string(&x[0]),
        //         x.iter().map(serialize_form_value).collect(),
        //     );
        // }
        // // FormValue::Dropdown(x) => todo!(),
        // FormValue::ListItem(x) => {
        //     SurveyPart::ListItem(get_bool(&x[0]), x[1..].iter().map(get_string).collect())
        // }
        // FormValue::TextInput(x) => SurveyPart::TextInput(x.to_owned().to_owned()),
        // // FormValue::Empty => todo!(),
        // // FormValue::Nothing => todo!(),
        // FormValue::Dropdown(x) => {
        //     SurveyPart::Dropdown(get_string(&x[0]), x[1..].iter().map(get_string).collect())
        // }
        _ => SurveyPart::Nothing,
    }
}

fn serialize_value(formparts: Vec<FormValue>) -> Vec<SurveyPart> {
    // fn serialize_form_value(pair: &FormValue) -> SurveyPart {
    //     match pair {
    //         FormValue::Title { text } => SurveyPart::Title(text.to_owned()),
    //         // FormValue::TextInput(x) => todo!(),
    //         // FormValue::Empty => todo!(),
    //         // FormValue::Nothing => todo!(),
    //         FormValue::Checkbox { properties } => {
    //             let question = match properties.get(0).unwrap() {
    //                 FormValue::QuestionText { text } => text.clone(),
    //             };
    //             let options: Vec<CheckboxItem> = properties[1..]
    //                 .iter()
    //                 .map(|formvalue| match formvalue {
    //                     FormValue::ListItem { properties } => {
    //                         let checked = match properties.get(0).unwrap() {
    //                             FormValue::CheckedStatus { value } => value.clone(),
    //                         };
    //                         let optiontext = match properties.get(1).unwrap() {
    //                             FormValue::QuestionText { text } => text.clone(),
    //                         };

    //                         return CheckboxItem {
    //                             checked,
    //                             text: optiontext,
    //                         };
    //                     }
    //                     _ => {
    //                         unreachable!()
    //                     }
    //                 })
    //                 .collect();

    //             SurveyPart::Checkbox(CheckboxQuestion {
    //                 options: options,
    //                 question: question,
    //             })
    //         }
    //         // get_bool(&x[0]),       // x[1..].iter().map(|x| serialize_form_value(x)).collect(),
    //         // FormValue::Radio(x) => {
    //         //     return SurveyPart::Radio(
    //         //         get_string(&x[0]),
    //         //         x.iter().map(serialize_form_value).collect(),
    //         //     );
    //         // }
    //         // // FormValue::Dropdown(x) => todo!(),
    //         // FormValue::ListItem(x) => {
    //         //     SurveyPart::ListItem(get_bool(&x[0]), x[1..].iter().map(get_string).collect())
    //         // }
    //         // FormValue::TextInput(x) => SurveyPart::TextInput(x.to_owned().to_owned()),
    //         // // FormValue::Empty => todo!(),
    //         // // FormValue::Nothing => todo!(),
    //         // FormValue::Dropdown(x) => {
    //         //     SurveyPart::Dropdown(get_string(&x[0]), x[1..].iter().map(get_string).collect())
    //         // }
    //         _ => SurveyPart::Nothing,
    //     }
    // }

    return formparts.iter().map(form_value_to_survey_part).collect();
}

// fn get_string(status: &FormValue) -> String {
//     println!("get_string: {:?}", status);
//     return match status {
//         FormValue::TextInput(x) => x.to_owned().to_owned().clone(),
//         FormValue::QuestionText(x) => x.to_owned().to_owned().clone(),
//         // FormValue::ListItem(x) => x[1],
//         _ => "Not implemented".to_string(),
//     };
// }

// fn get_bool(status: &FormValue) -> bool {
//     println!("get_bool: {:?}", status);
//     return match status {
//         FormValue::CheckedStatus(x) => x.to_owned(),
//         _ => false,
//     };
// }

#[derive(Deserialize, Serialize, Clone, Debug)]
struct RadioQuestion {
    question: String,
    options: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct CheckboxQuestion {
    question: String,
    options: Vec<CheckboxItem>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
struct CheckboxItem {
    checked: bool,
    text: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SurveyPart {
    Title(String),
    Radio(RadioQuestion),
    Checkbox(CheckboxQuestion),
    Dropdown {
        question: String,
        options: Vec<String>,
    },
    TextInput {
        question: String,
    },
    Nothing,
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
    // let (block_type, survey_part) = match formvalue {
    //     FormValue::Title { text } => (BlockType::Title, SurveyPart::Title(text.clone())),
    //     // FormValue::TextInput { text } => BlockType::TextInput,
    //     // FormValue::Radio { properties } => BlockType::Radio,
    //     // FormValue::Dropdown { properties } => BlockType::Dropdown,
    //     // FormValue::Submit { text } => BlockType::Submit,
    //     // FormValue::TextArea { text } => BlockType::Textarea,
    //     FormValue::Checkbox { properties } => (BlockType::Checkbox, form_value_to_survey_part(prop)),
    //     _ => BlockType::Empty,
    // };

    let survey_part = form_value_to_survey_part(formvalue);

    return Block {
        id: NanoId::new(),
        index: 0.0,
        properties: survey_part,
        block_type: BlockType::Checkbox,
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

        println!("formvalue: {:?}", formvalue);
        // match formvalue {
        //     FormValue::Title { text } => {
        //         survey.title = text.to_string();
        //     }
        //     // FormValue::TextInput { text } => todo!(),
        //     // FormValue::Empty => todo!(),
        //     FormValue::Nothing => {}
        //     FormValue::Checkbox { properties } => {
        //         let mut question = Question {
        //             id: nanoid_gen(NANOID_LEN),
        //             value: "".to_string(),
        //             options: vec![],
        //             r#type: QuestionType::Checkbox,
        //             created_on: Utc::now().to_string(),
        //             modified_on: Utc::now().to_string(),
        //         };
        //         for o in properties {
        //             match o {
        //                 FormValue::ListItem { properties } => {
        //                     match properties.get(1).unwrap() {
        //                         FormValue::CheckedStatus { value } => {}
        //                         FormValue::QuestionText { text } => {
        //                             question.options.push(QuestionOption {
        //                                 text: text.to_string(),
        //                                 id: "fixme".to_string(),
        //                             })
        //                         }
        //                         _ => unreachable!(),
        //                     };
        //                 }
        //                 FormValue::QuestionText { text } => question.value = text.to_owned(),
        //                 _ => unreachable!(),
        //             }
        //         }
        //         survey.questions.push(question);
        //     }
        //     FormValue::Radio { properties } => todo!(),
        //     FormValue::Dropdown { properties } => todo!(),
        //     FormValue::Submit { text } => {}
        //     FormValue::TextArea { text } => todo!(),
        //     _ => {}
        // };
    }
    return survey;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(untagged)]
// #[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum FormValue {
    // #[serde(flatten)]
    Title { text: String },
    TextInput { text: String },
    Empty,
    Nothing,
    Radio { properties: Vec<FormValue> },
    Dropdown { properties: Vec<FormValue> },
    ListItem { properties: Vec<FormValue> },
    QuestionText { text: String },
    Submit { text: String },
    TextArea { text: String },
    CheckedStatus { value: bool },
    DefaultValue { text: String },
    Checkbox { properties: Vec<FormValue> },
}

#[cfg(test)]
mod tests {
    // use super::do_thing;

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
