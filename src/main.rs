use regex::Regex;

#[derive(Clone, Debug)]
struct Survey {
    survey_id: i32,
    content: String,
    questions: Vec<Question>,
}
#[derive(Clone, Debug)]
struct Question {
    id: i32,
    text: String,
    options: Vec<String>,
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

fn parse_markdown_blocks() {
    let markdown = include_str!("../test_file.md").to_string();
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
    let mut currline = lines.next().unwrap();

    loop {
        let mut q_num = format!("{}. ", current);
        println!("{}", currline);
        // Is a question
        if currline.starts_with(q_num.as_str()) {
            current += 1;

            // current_question = currline.trim_start_matches(q_num.as_str()).to_owned();
            current_question = parse_question_text(currline).to_owned();

            currline = lines.next().unwrap();
            println!("{}", currline);
            while currline.starts_with("  ") {
                options.push(parse_question_text(currline).to_owned());
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
}

fn parse_question_text(line: &str) -> &str {
    line.split_once(". ").unwrap().1
}

enum MarkdownElement {
    Heading,
    List,
    ListItem,
    Nothing,
}

fn main() {
    parse_markdown_blocks();
}
