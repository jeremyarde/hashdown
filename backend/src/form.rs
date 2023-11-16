use std::io::Empty;

use anyhow::anyhow;
use pest::error::{Error, LineColLocation};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::Question;

#[derive(Parser)]
#[grammar = "form.pest"]
struct FormParser;

#[derive(Debug)]
enum FormValue<'a> {
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

pub fn do_thing() {
    let data = parse_markdown_text();
    match data {
        Ok(x) => {
            println!("{:#?}", &x);
        }
        Err(x) => {
            println!(
                "Line (Row, Col)={:?}, with content {:?} is not formatted properly.",
                x.line_col,
                x.line(),
            );
        }
    }
}

pub fn parse_markdown_text() -> anyhow::Result<Vec<FormValue<'static>>, Error<Rule>> {
    use pest::iterators::Pair;

    let formtext = match FormParser::parse(Rule::form, include_str!("../formexample.md")) {
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
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::do_thing;

    #[test]
    fn test_parse() {
        do_thing()
    }
}
