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
    let set_answer = use_state(cx, || HashMap::<String, Answer>::new());

    let update_answer = move |id: String, value: String| {
        answers_state.write().entry(id).and_modify(|e| {

        });
        // match question.r#type {
        //     QuestionType::Radio => {
        //         let new_answer = Answer::Radio { id: question.id.clone(), value: option.text.clone() };
        //         answer_state.write().insert(question.id.clone(), new_answer);
        //     }
        //     QuestionType::Checkbox => {
        //         match answer_state.write().get_mut(&question.id) {
        //             Some(val) => {
        //                 // let thing = x;
        //                 // let new_answer = Answer::MultipleChoice { id: question.id.clone(), value: vec![option.text.clone()] };
        //                 match val {
        //                     Answer::MultipleChoice {id, value} => {value.push(option.text.clone());}
        //                     // Answer::Radio {id, value} => {value = option.text.clone();}
        //                     _ => {}
        //                 }
        //                 // v.insert(new_answer);
        //                 // answer_state.write().insert(question.id.clone(), new_answer);
        //             }
        //             None => {
        //                 let new_answer = Answer::MultipleChoice { id: question.id.clone(), value: vec![option.text.clone()] };
        //                 answer_state.write().insert(question.id.clone(), new_answer);

        //             }
        //         }
        //     }
        //     _ => { info!("Question type not supported"); }
        //     // Answer::MultipleChoice { id: id.to_owned(), value: value.to_owned() },
        //     // Answer::Radio { id, value } => todo!(),
        // };
        
        info!("Triggered update_answer: {:?}, {:?}", id, value);
    };

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
                        // question.clone(),
                        update_answer_callback: update_answer,
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

// pub fn RenderSurvey<'a>(cx: Scope, survey_to_render: &'a ParsedSurvey) -> Element {

#[derive(Props, PartialEq)]
struct QuestionProps<'a> {
    update_answer_callback: Box<dyn Fn(String, String)>,
    question: &'a Question,
}

// static ANSWERS: Atom<HashMap<String, Answer>> = |_| HashMap::new();
// #[inline_props]
fn Question<'a>(cx: Scope<'a, QuestionProps<'a>> 

    // question: Question, 
    // set_answer: Box<dyn Fn(&Question, Answer)>
    // answer: &'a mut Answer,
    // update_answer_callback: Box<dyn Fn(String, String)>
) -> Element {
    // let answer_state = use_atom_ref(cx, ANSWERS);
    // let answers = use_state(cx, || vec![]);
    // let set_answer = use_set(cx, ANSWERS);

    // answer_state.modify(|curr| {
    //     let new = HashSet::new();
    //     // new.insert(curr);

        
    //     curr.insert(Answer::Radio { id: "".to_string(), value: "".to_string()}); 

    //     new
    // });

    // answer_state.write().entry("test").and_modify(|e| e.)

    cx.render(rsx!(div {
        match cx.props.question.r#type {
            QuestionType::Checkbox | QuestionType::Radio => {
            let value = cx.props.question.options.iter().enumerate().map(|(i, option): (usize, &QuestionOption)| {
                rsx!(
                    li {
                        input {
                            r#type: if cx.props.question.r#type == QuestionType::Checkbox { "checkbox"} else {"radio"},
                            // r#type: question_type,
                            // value: "{option.text}",
                            id: "{option.id}_{i}",
                            name: "{cx.props.question.id}",
                            onchange: move |evt| {
                                info!("Checkbox/Radio change event - {:?} > {:?}: {:?}", cx.props.question.id, option.id, evt);
                                // cx.props.update_answer_callback("test".to_string(), "test".to_string())
                                (cx.props.update_answer_callback)(option.id.clone(), option.text.clone());
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
            _ => rsx!(div{"not supported"})
        }
    }))
}
