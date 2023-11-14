use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "form.pest"]
struct FormParser;

pub fn do_thing() {
    let results = FormParser::parse(Rule::form, include_str!("../formexample.md")).unwrap();
    let mut survey_parts = vec![];

    for pair in results {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        // println!("Text:    {}", pair.as_str());

        for inner in pair.into_inner() {
            println!("Rule:    {:?}", inner.as_rule());
            println!("Span:    {:?}", inner.as_span());

            match inner.clone().as_rule() {
                // Rule::EOI => todo!(),
                // Rule::form => todo!(),
                // Rule::emptyline => todo!(),
                Rule::header => survey_parts.push(inner.as_str()),
                Rule::block => {
                    inner.clone().into_inner().for_each(|x| match x.as_rule() {
                        Rule::text_input => survey_parts.push(inner.as_str()),
                        Rule::textarea => survey_parts.push(inner.as_str()),
                        Rule::checkbox => survey_parts.push(inner.as_str()),
                        Rule::radio => survey_parts.push(inner.as_str()),
                        Rule::dropdown => survey_parts.push(inner.as_str()),
                        Rule::submit => survey_parts.push(inner.as_str()),
                        Rule::comment => survey_parts.push(inner.as_str()),
                        _ => {}
                    });
                }
                // Rule::EOI => todo!(),
                // Rule::form => todo!(),
                // Rule::emptyline => todo!(),
                // Rule::text_input => todo!(),
                // Rule::textarea => todo!(),
                // Rule::checkbox => todo!(),
                // Rule::radio => todo!(),
                // Rule::dropdown => todo!(),
                // Rule::submit => todo!(),
                // Rule::comment => todo!(),
                // Rule::question_text => todo!(),
                // Rule::listitem => todo!(),
                // Rule::SPACE => todo!(),
                // Rule::question_text => todo!(),
                // Rule::listitem => todo!(),
                // Rule::SPACE => todo!(),
                _ => {}
            }
            // println!("Text:    {}", inner.as_str());
        }
    }
    println!("{:#?}", survey_parts);
}

#[cfg(test)]
mod tests {
    use super::do_thing;

    #[test]
    fn test_parse() {
        do_thing()
    }
}
