use regex::Regex;

#[derive(Clone, Debug)]
struct Survey {
    survey_id: i32,
    content: String,
    questions: Vec<Question>,
}
#[derive(Clone, Debug)]
pub struct Question {
    id: i32,
    pub text: String,
    pub options: Vec<String>,
}

#[derive(Clone, Debug)]
struct Answer {
    survey_id: i32,
    name: String,
    question_number: i32,
    answer: String,
}
#[derive(Clone, Debug)]
enum Types {
    checkbox,
    radio,
    text,
}

pub type Questions = Vec<Question>;

pub fn parse_markdown_blocks(markdown: String) -> Questions {
    // let markdown = include_str!("../test_file.md").to_string();
    let questions = Regex::new(r"(?m)^(\d). (.*)$").unwrap();
    let locations = questions.captures_iter(&markdown);
    // for x in locations {
    //     println!("{:#?}", x);
    // }
    let mut questions = vec![];

    let mut current_question: String;
    let mut options: Vec<String> = vec![];
    let mut question_id = 1;

    let mut current = 1;
    // for line in markdown.lines() {
    let mut lines = markdown.lines();
    let mut currline = match lines.next() {
        Some(x) => x,
        None => return vec![],
    };

    loop {
        let mut q_num = format!("{}. ", current);
        println!("{}", currline);
        // Is a question
        if currline.starts_with(q_num.as_str()) {
            current += 1;

            // current_question = currline.trim_start_matches(q_num.as_str()).to_owned();
            current_question = match parse_question_text(currline).to_owned() {
                Some(x) => x.to_string(),
                None => "".to_string(),
            };
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
                match parse_question_text(currline).to_owned() {
                    Some(x) => options.push(x.to_string()),
                    None => {}
                }
                currline = match lines.next() {
                    Some(x) => x,
                    None => break,
                };
            }

            questions.push(Question {
                id: question_id,
                text: current_question,
                options: options,
            });
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
    questions
}

fn parse_question_text(line: &str) -> Option<&str> {
    match line.split_once(". ") {
        Some(x) => Some(x.1),
        None => None,
    }
}

enum MarkdownElement {
    Heading,
    List,
    ListItem,
    Nothing,
}

#[cfg(test)]
mod tests {
    use crate::parse_markdown_blocks;

    #[test]
    fn test() {
        let teststring = "1. this is a test\n  ";

        let content = String::from(teststring);
        let result = parse_markdown_blocks(content);
        print!("test result: {:?}\n", result);
    }
}
