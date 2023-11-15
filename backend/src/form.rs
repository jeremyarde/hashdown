use std::io::Empty;

use pest::Parser;
use pest_derive::Parser;

use crate::Question;

#[derive(Parser)]
#[grammar = "form.pest"]
struct FormParser;

#[derive(Debug)]
enum FormValue<'a> {
    Title(&'a str),
    Text(&'a str),
    Empty,
    Nothing,
    Checkbox(Vec<FormValue<'a>>),
    Radio(Vec<FormValue<'a>>),
    Dropdown(Vec<FormValue<'a>>),
    ListItem(Vec<FormValue<'a>>),
    QuestionText(&'a str),
    Submit(&'a str),
}

// pub fn serialize_formvalue(val: &FormValue() {
//     use FormValue::*;

//     match val {
//         Title(s) => ,
//         Text(s) => todo!(),
//     }
// }

pub fn do_thing() {
    use pest::iterators::Pair;

    let formtext = FormParser::parse(Rule::form, include_str!("../formexample.md")).unwrap();

    fn parse_value(pair: Pair<Rule>) -> FormValue {
        let rule = pair.as_rule();
        let val = pair.as_str();
        match pair.as_rule() {
            // Rule::EOI => todo!(),
            // Rule::emptyline => todo!(),
            Rule::header => FormValue::Title(pair.into_inner().as_str()),
            // Rule::block => todo!(),
            Rule::text_input => FormValue::Text(pair.into_inner().as_str()),
            // Rule::textarea => todo!(),
            Rule::checkbox => FormValue::Checkbox(pair.into_inner().map(parse_value).collect()),
            Rule::radio => FormValue::Radio(pair.into_inner().map(parse_value).collect()),
            Rule::dropdown => FormValue::Dropdown(pair.into_inner().map(parse_value).collect()),
            Rule::submit => FormValue::Submit(pair.as_str()),
            // Rule::comment => todo!(),
            Rule::question_text => FormValue::QuestionText(pair.as_str()),
            Rule::listitem => FormValue::ListItem(pair.into_inner().map(parse_value).collect()),
            // Rule::SPACE => todo!(),
            Rule::form | Rule::block | _ => FormValue::Nothing,
            // _ => FormValue::Empty,
        }
    }
    let data = formtext
        .map(|pair| parse_value(pair))
        .collect::<Vec<FormValue>>();
    // let formvalue = parse_value(formtext);

    println!("{:#?}", data);
    // dbg!(parse_value(formvalue))
}

#[cfg(test)]
mod tests {
    use super::do_thing;

    #[test]
    fn test_parse() {
        do_thing()
    }
}
