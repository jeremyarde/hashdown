use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_state, use_init_atom_root, use_set, Atom, AtomRef};
use log::info;
use markdownparser::{ParsedSurvey, Question, QuestionOption, QuestionType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::mainapp::{AppError, AppState, LoginPayload, UserContext, APP, EDITOR, SURVEY};

#[derive(Deserialize, Serialize, Debug, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Answer {
    // MultipleChoice { id: String, value: Vec<String> },
    Radio { id: String, value: String },
}

// static ANSWERS: Atom<HashSet<Answer>> = |_| HashSet::new();
// static ANSWERS: AtomRef<HashMap<String, Answer>> = |_| HashMap::new();

#[inline_props]
pub fn RenderSurvey(cx: Scope) -> Element {
    // let app_state = use_atom_ref(cx, APP);
    let editor_state = use_atom_state(cx, &EDITOR);
    let survey_state = use_atom_state(cx, &SURVEY);

    let submit_survey = move |evt: FormEvent| {
        cx.spawn({
            // to_owned![app_state];
            async move {
                let url = "/answer";
                let client_url = format!("http://{}{}", "localhost:3000", url);

                println!("Sending req to: {client_url}");

                // let request: LoginPayload = LoginPayload {
                //     email: evt.values["email"].get(0).unwrap().to_owned(),
                //     password: evt.values["password"].get(0).unwrap().to_owned(),
                // };
                let formdata = evt.values.clone();
                info!("submit form details: {:?}", formdata);

                // let resp = reqwest::Client::new()
                // .get("http://localhost:3000/surveys")
                // .header("x-auth-token", token)
                // .send()
                // .await;

                // let response = client
                //     .post(&client_url)
                //     .json(&request)
                //     .send()
                //     .await
                //     .expect("Should recieve response from app");
            }
        });
    };
    cx.render(rsx! {
        div { class: "flex flex-col",
            form {
                // action: "http://localhost:3000/surveys/{survey_to_render.survey.metadata.id}",
                // action: "http://localhost:3000/surveys/test",
                // enctype: "application/x-www-form-urlencoded",
                // method: "post",
                // prevent_default: "onsubmit",
                onsubmit: move |evt| {
                    info!("submitting survey result: {:?}", evt.values);
                },
                onchange: move |evt| {
                    info!("form: {:#?}", evt.data);
                },
                h1 { "title: {survey_state.survey.title:?}" }
                // app_state.read().survey.survey.questions.iter().map(|question| {
                survey_state.survey.questions.iter().map(|question| {
                    info!("curr question: {:?}" ,question);
                    // let curr_state = answer_state.get().get(&question.id.clone()).unwrap();
                    rsx!{

                    Question {
                        question: question,
                    }

                }}),
                button { class: "", onsubmit: move |evt| submit_survey(evt), r#type: "submit", "Submit" }
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct QuestionProps<'a> {
    question: &'a Question,
}

fn Question<'a>(cx: Scope<'a, QuestionProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            match cx.props.question.r#type {
            QuestionType::Checkbox | QuestionType::Radio => {
                let value = cx.props.question.options.iter().enumerate().map(|(i, option): (usize, &QuestionOption)| {
                    rsx!(
                        li {
                            input {
                                r#type: if cx.props.question.r#type == QuestionType::Checkbox { "checkbox"} else {"radio"},
                                value: "{option.text}",
                                id: "{option.id}_{i}",
                                name: "{cx.props.question.id}",
                                onchange: move |evt| {
                                    info!("Checkbox/Radio change event - {:?} > {:?}: {:?}", cx.props.question.id, option.id, evt);
                                },
                            }
                            label {
                                r#for:"{option.id}_{i}",
                                "{option.id}_{i}: {option.text:?}"
                            }
                        }
                    )
                    });

                rsx!(
                    h3 {
                        "{cx.props.question.id}: {cx.props.question.value}"
                    }
                    value
                )
            }
            QuestionType::Text => {
                rsx!(li {
                    label {
                        r#for: "{cx.props.question.id}",
                        "{cx.props.question.id}: {cx.props.question.value}"
                    }
                    input {
                        // value: "{cx.props.question.value}",
                        id: "{cx.props.question.id}",
                        name: "{cx.props.question.id}",
                    }
                })
            }
            _ => rsx!(div{"not supported"})
        }
        }
    ))
}
