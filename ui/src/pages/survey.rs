use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use fermi::{use_atom_state, Atom, use_set, use_atom_ref, AtomRef, use_init_atom_root};
use log::info;
use markdownparser::{ParsedSurvey, Question, QuestionOption, QuestionType};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::mainapp::{AppError, AppState, LoginPayload, UserContext, APP};

#[derive(Deserialize, Serialize, Debug, Hash, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Answer {
    MultipleChoice { id: String, value: Vec<String> },
    Radio { id: String, value: String },
}

impl PartialEq for Answer {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // (Answer::MultipleChoice { id, value }, Answer::Radio { id, value }) => todo!(),
            // (Answer::Radio { id, value }, Answer::MultipleChoice { id, value }) => todo!(),
            (Answer::Radio { id, .. } 
                | Answer::MultipleChoice { id, .. }, 
            Answer::Radio { id: idy, .. }
                | Answer::MultipleChoice { id: idy, .. }) => id == idy,
            (_, _) => {false}
        }
    }
}

// static ANSWERS: Atom<HashSet<Answer>> = |_| HashSet::new();
static ANSWERS: AtomRef<HashMap<String, Answer>> = |_| HashMap::new();


#[inline_props]
pub fn RenderSurvey<'a>(cx: Scope, survey_to_render: &'a ParsedSurvey) -> Element {
    let app_state = use_atom_state(cx, APP);
    let answers_state = use_atom_ref(cx, ANSWERS);

    // Build the state for the answers
    // for x in app_state.survey.survey.questions.iter() {
    //     if x.r#type == QuestionType::Checkbox || x.r#type == QuestionType::Radio {
    //         let new_answer: Option<Answer> = match x.r#type {
    //             QuestionType::Radio => {
    //                 Some(Answer::Radio { id: x.id.clone(), value: "".to_string() })
    //             }
    //             QuestionType::Checkbox => {
    //                 Some(Answer::MultipleChoice { 
    //                     id: x.id.clone(), 
    //                     value: vec![]
    //                 })
    //             },
    //             _ => {None}
    //         };
    //         if new_answer.is_some() {
    //             answers_state.write().insert(x.id.to_string(), {
    //                 new_answer.unwrap()
    //             });
    //         }
    //     }
    // }

    info!("answers state: {:#?}", answers_state.read());
            // Answer::Radio { id: "", value: () })
    
    // let mut answer_state = use_atom_state(cx, ANSWERS);

    
    // answer_state.insert(Answer::Radio { id: "".to_string(), value: "".to_string() });
    // let set_answer = move |curr: &Question, new_value: Answer| {
    //     answer_state.insert(curr.id, new_value);
    // };

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
                // action: "http://localhost:3000/surveys/{app_state.survey.metadata.id}",
                // enctype: "application/x-www-form-urlencoded",
                // method: "post",
                prevent_default: "onsubmit",
                onsubmit: move |evt| {
                    info!("submitting survey result: {:?}", evt.values);
                    post_questions(evt.values.clone(), app_state.client.clone());
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
                    // fieldset {
                    //     legend {
                    //         "question text: {question.value}"
                    //     }
                    //     ul {
                    //         // question.options.iter().enumerate().map(|(i, option): (usize, &QuestionOption)| {
                    //         rsx!{
                    //             Questions{question: &question}
                    //         }
                    //     }
                    // }
                    // Radio {question: question.clone()}
                    Question {
                        question: question.clone(), 
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

// pub fn RenderSurvey<'a>(cx: Scope, survey_to_render: &'a ParsedSurvey) -> Element {

// static ANSWERS: Atom<HashMap<String, Answer>> = |_| HashMap::new();
#[inline_props]
fn Question(cx: Scope, question: Question, 
    // set_answer: Box<dyn Fn(&Question, Answer)>
) -> Element {
    let answer_state = use_atom_ref(cx, ANSWERS);
    // let answers = use_state(cx, || vec![]);
    // let set_answer = use_set(cx, ANSWERS);

    // answer_state.modify(|curr| {
    //     let new = HashSet::new();
    //     // new.insert(curr);

        
    //     curr.insert(Answer::Radio { id: "".to_string(), value: "".to_string()}); 

    //     new
    // });

    cx.render(rsx!(div {
        match question.r#type {
            QuestionType::Checkbox | QuestionType::Radio => {
            let value = question.options.iter().enumerate().map(|(i, option): (usize, &QuestionOption)| {
                rsx!(
                    li {
                        input {
                            r#type: if question.r#type == QuestionType::Checkbox { "checkbox"} else {"radio"},
                            // r#type: question_type,
                            // value: "{option.text}",
                            id: "{option.id}_{i}",
                            name: "{question.id}",
                            onchange: move |evt| {
                                info!("Checkbox/Radio change event: {:?}", evt);


                                match question.r#type {
                                    QuestionType::Radio => {
                                        let new_answer = Answer::Radio { id: question.id.clone(), value: option.text.clone() };
                                        answer_state.write().insert(question.id.clone(), new_answer);
                                    }
                                    QuestionType::Checkbox => {
                                        match answer_state.write().get_key_value(&question.id) {
                                            Some(x) => {
                                                let _ = x;
                                                let new_answer = Answer::MultipleChoice { id: question.id.clone(), value: vec![option.text.clone()] };
                                                answer_state.write().insert(question.id.clone(), new_answer);

                                            }
                                            None => {
                                                let new_answer = Answer::MultipleChoice { id: question.id.clone(), value: vec![option.text.clone()] };
                                                answer_state.write().insert(question.id.clone(), new_answer);

                                            }
                                        }
                                    }
                                    _ => { info!("Question type not supported"); }
                                    // Answer::MultipleChoice { id: id.to_owned(), value: value.to_owned() },
                                    // Answer::Radio { id, value } => todo!(),
                                };
                                // let mut new_answers = answer_state.write();
                                // new_answers.write().insert(question.id.clone(), new_answer);
                                // let new_asnwers = HashMap::new();

                                // set_answer(new_answers);
                                // answer_state.modify(|curr| {
                                //     let new: HashSet<Answer> = HashSet::new();
                                //     // new.insert(curr);
                                //     // curr.insert(Answer::Radio { id: "".to_string(), value: "".to_string()}); 
                                //     // new.extend(curr.clone().to_owned());

                                //     let _ = curr.clone();
                                //     new
                                // });
                                // state.get()
                                // *curr_state.insert(question.id, Answer::Radio { id: question.id.clone(), value: evt.value }).unwrap();
                                // answer_state.modify(|mut curr| {
                                //     curr.insert(question.id, Answer::Radio { id: question.id.clone(), value: evt.value }).unwrap();
                                // });
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
                    "{question.id}: {question.value}"
                }
                value
            )
            } 
            _ => rsx!(div{"not supported"})
        }
    }))
}
