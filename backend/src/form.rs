use anyhow::anyhow;
use pest::error::{Error, LineColLocation};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::info;

use crate::Question;

#[derive(Parser)]
#[grammar = "form.pest"]
struct FormParser;

#[derive(Debug, Serialize, Deserialize)]
pub enum QuestionType {
    Checkbox,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FormValue {
    Title(String),
    TextInput(String),
    Empty,
    Nothing,
    Checkbox(Vec<FormValue>),
    Radio(Vec<FormValue>),
    Dropdown(Vec<FormValue>),
    ListItem(Vec<FormValue>),
    QuestionText(String),
    Submit(String),
    TextArea(String),
    CheckedStatus(bool),
    DefaultValue(String),
}

pub fn parse_markdown_text(contents: &str) -> anyhow::Result<Vec<FormValue>, Error<Rule>> {
    use pest::iterators::Pair;

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
            Rule::header => FormValue::Title(pair.into_inner().as_str().to_string()),
            Rule::text_input => FormValue::TextInput(pair.into_inner().as_str().to_string()),
            Rule::checkbox => FormValue::Checkbox(pair.into_inner().map(parse_value).collect()),
            Rule::radio => FormValue::Radio(pair.into_inner().map(parse_value).collect()),
            Rule::dropdown => FormValue::Dropdown(pair.into_inner().map(parse_value).collect()),
            Rule::submit => FormValue::Submit(pair.as_str().to_string()),
            // Rule::comment => todo!(),
            Rule::question_text => FormValue::QuestionText(pair.as_str().to_string()),
            Rule::listitem => FormValue::ListItem(pair.into_inner().map(parse_value).collect()),
            Rule::unchecked => FormValue::CheckedStatus(false),
            Rule::checked => FormValue::CheckedStatus(true),
            Rule::inner_default_value => FormValue::DefaultValue(pair.as_str().to_string()),
            Rule::EOI => FormValue::Nothing,
            Rule::textarea => FormValue::TextArea(pair.as_str().to_string()),
            Rule::comment
            | Rule::SPACE
            | Rule::emptyline
            | Rule::form
            | Rule::block
            | Rule::default_value => {
                unreachable!()
            }
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

fn serialize_value(formparts: Vec<FormValue>) -> Vec<SurveyPart> {
    fn serialize_form_value(pair: &FormValue) -> SurveyPart {
        match pair {
            FormValue::Title(x) => SurveyPart::Title(x.to_owned().to_owned()),
            // FormValue::TextInput(x) => todo!(),
            // FormValue::Empty => todo!(),
            // FormValue::Nothing => todo!(),
            FormValue::Checkbox(x) => SurveyPart::Checkbox(
                get_bool(&x[0]),
                x[1..].iter().map(|x| serialize_form_value(x)).collect(),
            ),
            FormValue::Radio(x) => {
                return SurveyPart::Radio(
                    get_string(&x[0]),
                    x.iter().map(serialize_form_value).collect(),
                );
            }
            // FormValue::Dropdown(x) => todo!(),
            FormValue::ListItem(x) => {
                SurveyPart::ListItem(get_bool(&x[0]), x[1..].iter().map(get_string).collect())
            }
            FormValue::TextInput(x) => SurveyPart::TextInput(x.to_owned().to_owned()),
            // FormValue::Empty => todo!(),
            // FormValue::Nothing => todo!(),
            FormValue::Dropdown(x) => {
                SurveyPart::Dropdown(get_string(&x[0]), x[1..].iter().map(get_string).collect())
            }
            _ => SurveyPart::Nothing,
        }
    }

    return formparts.iter().map(serialize_form_value).collect();
}

fn get_string(status: &FormValue) -> String {
    println!("get_string: {:?}", status);
    return match status {
        FormValue::TextInput(x) => x.to_owned().to_owned().clone(),
        FormValue::QuestionText(x) => x.to_owned().to_owned().clone(),
        // FormValue::ListItem(x) => x[1],
        _ => "Not implemented".to_string(),
    };
}

fn get_bool(status: &FormValue) -> bool {
    println!("get_bool: {:?}", status);
    return match status {
        FormValue::CheckedStatus(x) => x.to_owned(),
        _ => false,
    };
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum SurveyPart {
    Title(String),
    Radio(String, Vec<SurveyPart>),
    ListItem(bool, String),
    Nothing,
    Checkbox(bool, Vec<SurveyPart>),
    Dropdown(String, Vec<String>),
    TextInput(String), // Title(&'a str),
                       // TextInput(&'a str),
                       // Empty,
                       // Nothing,
                       // Checkbox(Vec<FormValue<'a>>),
                       // Radio(Vec<FormValue<'a>>),
                       // Dropdown(Vec<FormValue<'a>>),
                       // ListItem(Vec<FormValue<'a>>),
                       // QuestionText(&'a str),
                       // Submit(&'a str),
                       // TextArea(&'a str),
                       // CheckedStatus(bool),
                       // DefaultValue(&'a str),
}

#[cfg(test)]
mod tests {
    // use super::do_thing;

    use serde_json::json;

    use crate::form::{parse_markdown_text, serialize_value};

    #[test]
    fn test_parse() {
        let res = parse_markdown_text(include_str!("../formexample.md"));
        // let res = do_thing();
        println!("{:#?}", res);

        let serialized = json!(res.unwrap());
        println!("{:#?}", serialized);
    }
}
