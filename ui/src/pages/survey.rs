use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use fermi::{use_atom_ref, use_atom_state, use_init_atom_root, use_set, Atom, AtomRef};
use log::info;
use markdownparser::{ParsedSurvey, Question, QuestionOption, QuestionType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::mainapp::{AppError, AppState, LoginPayload, UserContext, APP};

#[derive(Deserialize, Serialize, Debug, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Answer {
    // MultipleChoice { id: String, value: Vec<String> },
    Radio { id: String, value: String },
}

// static ANSWERS: Atom<HashSet<Answer>> = |_| HashSet::new();
static ANSWERS: AtomRef<HashMap<String, Answer>> = |_| HashMap::new();

#[inline_props]
pub fn RenderSurvey<'a>(cx: Scope, survey_to_render: &'a ParsedSurvey) -> Element {
    let app_state = use_atom_state(cx, APP);
    let answers_state = use_atom_ref(cx, ANSWERS);
    let set_answer = use_state(cx, || HashMap::<String, Answer>::new());

    info!("answers state: {:#?}", answers_state.read());

    let post_questions = move |content, client: Client| {
        cx.spawn({
            to_owned![app_state];

            let curr_survey_id = app_state.survey.metadata.id.to_string();
            async move {
                info!("Attempting to save questions...");

                // info!("Publishing content, app_state: {app_state:?}");
                // info!("answers state: {:#?}", &answers_state.read());
                // let formdata = FormData::from(content);
                // info!("Questions save: {:?}", question_state);
                match client
                    .post(format!("http://localhost:3000/surveys/{curr_survey_id}"))
                    .json(&json!(content))
                    // .bearer_auth(token.clone())
                    // .header("x-auth-token", token)
                    .send()
                    .await
                {
                    Ok(x) => {
                        info!("success: {x:?}");
                        info!("should show toast now");
                        // toast_visible.set(true);
                    }
                    Err(x) => info!("error: {x:?}"),
                };
            }
        })
    };

    cx.render(rsx! {
        div {
            class: "survey",
            form {
                action: "http://localhost:3000/surveys/{app_state.survey.metadata.id}",
                // action: "http://localhost:3000/surveys/test",
                // enctype: "application/x-www-form-urlencoded",
                method: "post",
                prevent_default: "onsubmit",
                onsubmit: move |evt| {
                    info!("submitting survey result: {:?}", evt.values);

                    let answers: Vec<Answer> = app_state.survey.survey.questions.iter().map(|question| {
                        // evt.values.get(&question.id);
                        Answer::Radio {
                            id:question.id.clone(), 
                            value: evt.values.get(&question.id).unwrap().to_owned()
                        }
                    }).collect();

                    info!("answers vec: {:?}", answers);

                    post_questions(answers, app_state.client.clone());

                    // evt.stop_propagation();
                },
                onchange: move |evt| {
                    info!("form: {:#?}", evt.data);
                    // info!("form testing deserialize: {:#?}", serde_json::Value::from_str(&evt.data.value));
                    info!("appstate: {:#?}", app_state.survey);
                    // evt
                },
                h1 {"title: {app_state.survey.survey.title:?}"}
                app_state.survey.survey.questions.iter().map(|question| {
                    info!("curr question: {:?}" ,question);
                    // let curr_state = answer_state.get().get(&question.id.clone()).unwrap();
                    rsx!{

                    Question {
                        // question.clone(),
                        // update_answer_callback: update_answer,
                        question: question,
                        // onupdate: update_answer
                        // answer: answers_state.write().get_mut(&question.id).unwrap()
                        // set_answer: set_answer
                    }

                }})
                button {
                    class: "publish-button",
                    r#type: "submit",
                    "Submit"
                }
            }
        }
    })
}

#[derive(Props, PartialEq)]
struct QuestionProps<'a> {
    question: &'a Question,
}

fn Question<'a>(
    cx: Scope<'a, QuestionProps<'a>>,
) -> Element {
    cx.render(rsx!(div {
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
    }))
}
