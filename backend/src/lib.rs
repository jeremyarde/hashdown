use anyhow::{anyhow, Result};
use nanoid::nanoid;
use regex::Regex;

const NANOID_LEN: usize = 12;
// const NANOID_ALPHA: [char; 36] = [
//     '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
//     'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
// ];
const NANOID_ALPHA: [char; 34] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(Clone, Debug)]
struct Survey {
    survey_id: i32,
    content: String,
    questions: Vec<Question>,
}
#[derive(Clone, Debug)]
pub struct Question {
    pub id: String,
    pub text: String,
    pub options: Vec<String>,
}

impl Question {
    fn from(q_text: &str, options: Vec<String>) -> Self {
        return Question {
            id: nanoid!(NANOID_LEN, &NANOID_ALPHA),
            text: q_text.to_string(),
            options,
        };
    }
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
    questions
}

enum LineType {
    Question,
    Option,
}

fn parse_markdown_v3(contents: String) -> Result<Questions> {
    let mut questions = Questions::new();
    let mut curr_question_text: &str = "";
    let mut curr_options: Vec<String> = vec![];
    let mut in_question = false;
    let mut last_line_type: LineType = LineType::Question;
    let mut question_num = 0;

    for line in contents.lines() {
        match (is_question(line), &last_line_type) {
            (true, LineType::Question) => {
                if question_num > 0 {
                    questions.push(Question::from(curr_question_text, curr_options.clone()));
                    curr_question_text = line;
                    curr_options.clear();
                }
                last_line_type = LineType::Question;
                curr_question_text = line;
            }
            (true, LineType::Option) => {
                questions.push(Question::from(curr_question_text, curr_options.clone()));
                curr_question_text = line;
                curr_options.clear();
                last_line_type = LineType::Question;
            }
            _ => {}
        }

        match (is_option(line), &last_line_type) {
            (true, LineType::Question) => {
                curr_options.push(line.clone().to_string());
                last_line_type = LineType::Option;
            }
            (true, LineType::Option) => {
                curr_options.push(line.clone().to_string());
                last_line_type = LineType::Option;
            }
            _ => {}
        }
    }

    // adding the last question
    questions.push(Question::from(curr_question_text, curr_options.clone()));

    Ok(questions)
}

fn is_question(line: &str) -> bool {
    !line.starts_with(" ") && line.starts_with(|c: char| c.eq(&'-') || c.is_digit(10))
}

fn is_option(line: &str) -> bool {
    let cleaned = line.clone().trim();
    line.starts_with(" ") && cleaned.starts_with(|c: char| c.eq(&'-') || c.is_digit(10))
}

fn is_valid_line(line: &str) -> bool {
    let line_copy = line.clone().trim_start();

    line_copy.starts_with(|c: char| c.eq(&'-') || c.is_digit(10))
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
    use crate::{parse_markdown_blocks, parse_markdown_v3};

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
}
