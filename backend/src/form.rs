use std::fmt;
use std::io::Empty;

use anyhow::anyhow;
use pest::error::{Error, LineColLocation};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use serde::{Deserialize, Serialize};

use crate::Question;

#[derive(Parser)]
#[grammar = "form.pest"]
struct FormParser;

#[derive(Debug)]
pub enum FormValue<'a> {
    Title(&'a str),
    TextInput(&'a str),
    Empty,
    Nothing,
    Checkbox(Vec<FormValue<'a>>),
    Radio(Vec<FormValue<'a>>),
    Dropdown(Vec<FormValue<'a>>),
    ListItem(Vec<FormValue<'a>>),
    QuestionText(&'a str),
    Submit(&'a str),
    TextArea(&'a str),
    CheckedStatus(bool),
    DefaultValue(&'a str),
}

impl<'a> Serialize for FormValue<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

pub fn do_thing() -> Vec<FormValue<'a>> {
    let data = parse_markdown_text(include_str!("../formexample.md"));
    match data {
        Ok(x) => {
            println!("{:#?}", &x);
            return x;
        }
        Err(x) => {
            println!(
                "Line (Row, Col)={:?}, with content {:?} is not formatted properly.",
                x.line_col,
                x.line(),
            );
            return vec![];
        }
    }
}

pub fn parse_markdown_text(contents: &str) -> anyhow::Result<Vec<FormValue>, Error<Rule>> {
    use pest::iterators::Pair;

    let formtext = match FormParser::parse(Rule::form, &contents) {
        Ok(x) => x,
        Err(x) => return Err(x),
    };

    fn parse_value(pair: Pair<Rule>) -> FormValue {
        let rule = pair.as_rule();
        let val = pair.as_str();
        println!("{:?}", rule);
        match pair.as_rule() {
            Rule::header => FormValue::Title(pair.into_inner().as_str()),
            Rule::text_input => FormValue::TextInput(pair.into_inner().as_str()),
            Rule::checkbox => FormValue::Checkbox(pair.into_inner().map(parse_value).collect()),
            Rule::radio => FormValue::Radio(pair.into_inner().map(parse_value).collect()),
            Rule::dropdown => FormValue::Dropdown(pair.into_inner().map(parse_value).collect()),
            Rule::submit => FormValue::Submit(pair.as_str()),
            // Rule::comment => todo!(),
            Rule::question_text => FormValue::QuestionText(pair.as_str()),
            Rule::listitem => FormValue::ListItem(pair.into_inner().map(parse_value).collect()),
            Rule::unchecked => FormValue::CheckedStatus(false),
            Rule::checked => FormValue::CheckedStatus(true),
            Rule::inner_default_value => FormValue::DefaultValue(pair.as_str()),
            Rule::EOI => FormValue::Nothing,
            Rule::textarea => FormValue::TextArea(pair.as_str()),
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
            // FormValue::Checkbox(x) => todo!(),
            FormValue::Radio(x) => SurveyPart::Radio(
                "testing title".to_owned(),
                x.iter().map(serialize_form_value).collect(),
            ),
            // FormValue::Dropdown(x) => todo!(),
            FormValue::ListItem(x) => {
                SurveyPart::ListItem(get_bool(&x[0]), x[1..].iter().map(get_string).collect())
            }
            FormValue::QuestionText(x) => todo!(),
            // FormValue::Submit(x) => todo!(),
            // FormValue::TextArea(x) => todo!(),
            // FormValue::CheckedStatus(x) => x,
            // FormValue::DefaultValue(x) => todo!(),
            _ => SurveyPart::Nothing,
        }
    }

    return formparts.iter().map(serialize_form_value).collect();
}

fn get_string(status: &FormValue) -> String {
    return match status {
        // FormValue::Title(_) => todo!(),
        FormValue::TextInput(x) => x.to_owned().to_owned(),
        // FormValue::Empty => todo!(),
        // FormValue::Nothing => todo!(),
        // FormValue::Checkbox(_) => todo!(),
        // FormValue::Radio(_) => todo!(),
        // FormValue::Dropdown(_) => todo!(),
        // FormValue::ListItem(_) => todo!(),
        // FormValue::QuestionText(_) => todo!(),
        // FormValue::Submit(_) => todo!(),
        // FormValue::TextArea(_) => todo!(),
        // FormValue::CheckedStatus(_) => todo!(),
        // FormValue::DefaultValue(_) => todo!(),
        _ => "".to_string(),
    };
}

fn get_bool(status: &FormValue) -> bool {
    return match status {
        // FormValue::Title(_) => todo!(),
        // FormValue::TextInput(_) => todo!(),
        // FormValue::Empty => todo!(),
        // FormValue::Nothing => todo!(),
        // FormValue::Checkbox(_) => todo!(),
        // FormValue::Radio(_) => todo!(),
        // FormValue::Dropdown(_) => todo!(),
        // FormValue::ListItem(_) => todo!(),
        // FormValue::QuestionText(_) => todo!(),
        // FormValue::Submit(_) => todo!(),
        // FormValue::TextArea(_) => todo!(),
        // FormValue::DefaultValue(_) => todo!(),
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
}

#[cfg(test)]
mod tests {
    use super::do_thing;

    #[test]
    fn test_parse() {
        let res = do_thing();
        println!(res);
    }
}
