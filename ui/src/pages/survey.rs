use dioxus::prelude::*;
use fermi::use_atom_state;
use log::info;
use markdownparser::{ParsedSurvey, QuestionOption, QuestionType};
use reqwest::Client;
use serde_json::{json, Value};

use crate::mainapp::{AppError, AppState, LoginPayload, UserContext, APP};


#[inline_props]
    pub fn RenderSurvey<'a>(cx: Scope, survey_to_render: &'a ParsedSurvey) -> Element {
        let app_state = use_atom_state(cx, APP);
        // let form_data: &fermi::AtomState<_> = use_atom_state(cx, FORM_DATA);

        // let questions = parse_markdown_v3(survey_to_render.plaintext.clone()).questions;
        // let questions = all_questions.get(0).unwrap();
        // let questions: Vec<Question> = vec![];
        // let survey_html
        let post_questions = move |content, client: Client| {
            cx.spawn({
                to_owned![app_state];

                if app_state.user.is_none() {
                    info!("user token is not set");
                    // Pop open the login ?
                    return;
                }
                let mut token  = app_state.user.clone().unwrap().token;
                token = token.trim_matches('"').to_string();

                let curr_survey_id = app_state.survey.metadata.id.to_string();
                async move {
                    info!("Attempting to save questions...");
                    info!("Publishing content, app_state: {app_state:?}");
                    // info!("Questions save: {:?}", question_state);
                    match client
                        .post(format!("http://localhost:3000/surveys/{curr_survey_id}"))
                        .json(&json!(content))
                        // .bearer_auth(token.clone())
                        .header("x-auth-token", token)
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


        // let curr_survey = app_state.curr_survey.clone();
        cx.render(rsx! {
                div {
                    class: "survey",
                    form {
                        prevent_default: "onsubmit",
                        onsubmit: move |evt| {
                            info!("submitting survey result: {:?}", evt.values);
                            // let formvalue = match evt.values.get(FORMINPUT_KEY).clone() {
                            //     Some(x) => {info!("form values: {x:?}"); x.clone()}
                            //     None => {"None".to_string()}
                            // };
                            // let formvalue = match evt.values.get(FORMINPUT_KEY).clone() {
                            //     Some(x) => {info!("found some data in the form"); x}
                            //     None => {"No data found"} 
                            // };
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
                        app_state.survey.survey.questions.iter().map(|question| rsx!{
                            fieldset {
                                legend {
                                    "question text: {question.value}"
                                }
                                ul {
                                    question.options.iter().enumerate().map(|(i, option): (usize, &QuestionOption)| {
                                        let test = "test";
                                        // rsx!(div{})
                                        
                                        let question_type = match question.r#type {
                                            QuestionType::Checkbox=>{"checkbox"}
                                            QuestionType::Radio => {"radio"},
                                            QuestionType::Text => {"text"},
                                            QuestionType::Number => {"number"},
                                            QuestionType::Email => "email",
                                            QuestionType::Date => "date"
                                        };
                                            
                                        rsx!{
                                            li {
                                                    input {
                                                        // r#type: if question.r#type == QuestionType::Checkbox { "checkbox"} else {"radio"},
                                                        r#type: question_type,
                                                        value: "{option.text:?}",
                                                        id: "{option.id}_{i}",
                                                        name: "{question.id}",
                                                    }
                                                    label {
                                                        r#for:"{option.id}_{i}",
                                                        "{option.id}_{i}: {option.text:?}"
                                                    }
                                            }
                                        } 
                                    })
                                }
                            }
                            
                        })
                        button {
                            class: "publish-button",
                            // r#type: "submit",
                            "Submit"
                        }
                    }
                }
        })
    }