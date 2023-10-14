use std::collections::HashMap;

use dioxus::{html::EventData, prelude::*};
use dioxus_router::prelude::*;

use log::info;
use markdownparser::{ParsedSurvey, Question, QuestionOption, QuestionType, Survey};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::mainapp::{AppError, AppState, LoginPayload, Route, UserContext};

#[derive(Deserialize, Serialize, Debug, Hash)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Answer {
    // MultipleChoice { id: String, value: Vec<String> },
    Radio { id: String, value: String },
}

#[component]
pub fn ListSurvey(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let error = use_state(cx, || "");
    let is_visible = use_state(cx, || false);

    // let mut surveys = use_ref(cx,  || vec![]);
    let surveys = use_future(cx, (&app_state.read().user), |(user)| async move {
        if user.is_none() {
            return HashMap::new();
        }
        let user_token = user.unwrap().token;
        // let url = "/responses";
        let client_url = format!("http://{}", "localhost:3000/surveys");

        println!("Sending req to: {client_url}");

        let resp = reqwest::Client::new()
            .get(client_url)
            // .json(&json!(request_form))
            .header("x-auth-token", user_token)
            .send()
            .await;

        info!("response from submit: {:?}", resp);

        let jsonresponse: HashMap<String, Vec<Value>> = resp.unwrap().json().await.unwrap();

        return jsonresponse;
    });

    let onsubmit = move |evt: EventData| {
        surveys.restart();
    };

    let get_surveys = move |evt: EventData, survey_id: String| {
        cx.spawn({
            to_owned![app_state, error, surveys];
            async move {
                let token = match app_state.read().user.clone() {
                    Some(user) => user.token,
                    None => {
                        error.set("Error listing surveys");
                        info!("Did not get user");
                        return;
                    }
                };
                // token = token.trim_matches('"').to_string();
                let resp = reqwest::Client::new()
                    .get(format!("http://localhost:3000/surveys/{}", survey_id))
                    .header("x-auth-token", token)
                    .send()
                    .await;

                match resp {
                    // Parse data from here, such as storing a response token
                    Ok(data) => {
                        info!("successful!");
                        // let jsondata = data.json().await.unwrap();

                        let jsonsurveys = data.json::<Value>().await.unwrap();

                        info!("get_surveys: {}", jsonsurveys);
                    }

                    //Handle any errors from the fetch here
                    Err(_err) => {
                        info!("failed - could not get data.")
                    }
                }
            }
        })
    };

    let logged_in = app_state.read().user.is_some();
    let curr_surveys: HashMap<String, Vec<Value>> = if surveys.value().is_some() {
        surveys.value().unwrap().to_owned()
    } else {
        HashMap::new()
    };

    render! {
        rsx!{
            div {
                if surveys.value().is_none() {
                        rsx!(div{"No surveys"})
                    } else {
                        rsx!{
                            curr_surveys.get("surveys").unwrap_or(&vec![]).iter().map(|survey: &Value| {
                                let survey_id = survey.get("survey_id").unwrap().as_str().unwrap().to_owned();
                                let version = survey.get("version").unwrap().as_str().unwrap().to_owned();

                                rsx!(
                                    div{
                                        "{survey_id} - version: {version}"
                                        li {
                                            Link { to: Route::RenderSurvey {survey_id}, "View survey" }
                                        }
                                    }
                                )
                            })
                        }
                    }
            }
        }
    }
}

#[component]
pub fn RenderSurvey(cx: Scope, survey_id: String) -> Element {
    // let app_state = use_atom_ref(cx, &APP);
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    // let editor_state = use_state(cx, || "".to_string());
    // let survey_state = use_state(cx, || Survey::new());
    let survey_state = use_future(
        cx,
        (survey_id, &app_state.read().user),
        |(survey_id, user)| async move {
            if user.is_none() {
                return None;
            }
            let user_token = user.unwrap().token;
            let url = "/responses";
            let client_url = format!("http://{}{}", "localhost:3000/surveys/", survey_id);

            println!("Sending req to: {client_url}");
            // formdata['survey_id'] =
            // let mut request_form: Value = json!({"survey_id": survey_id, });

            let resp = reqwest::Client::new()
                .get(client_url)
                // .json(&json!(request_form))
                .header("x-auth-token", user_token)
                .send()
                .await;

            info!("response from submit: {:?}", resp);

            let jsonresponse = resp.unwrap().json().await.unwrap();
            let survey: ParsedSurvey = serde_json::from_value(jsonresponse).unwrap();
            return Some(survey);
        },
    );

    let submit_survey = move |evt: FormEvent, survey_id: String, user_token: String| {
        cx.spawn({
            // to_owned![app_state];
            async move {
                let url = "/responses";
                let client_url = format!("http://{}{}", "localhost:3000", url);

                println!("Sending req to: {client_url}");

                // let request: LoginPayload = LoginPayload {
                //     email: evt.values["email"].get(0).unwrap().to_owned(),
                //     password: evt.values["password"].get(0).unwrap().to_owned(),
                // };
                let mut formdata = evt.values.clone();
                info!("submit form details: {:?}", formdata);
                // formdata['survey_id'] =
                let mut request_form: Value =
                    json!({"survey_id": survey_id, "responses": formdata});

                let resp = reqwest::Client::new()
                    .post(client_url)
                    .json(&json!(request_form))
                    .header("x-auth-token", user_token)
                    .send()
                    .await;

                info!("response from submit: {:?}", resp);

                // let response = client
                //     .post(&client_url)
                //     .json(&request)
                //     .send()
                //     .await
                //     .expect("Should recieve response from app");
            }
        });
    };

    render! {
        div { class: "flex flex-col",
            form {
                // action: "http://localhost:3000/surveys/{survey_to_render.survey.metadata.id}",
                // action: "http://localhost:3000/surveys/test",
                // enctype: "application/x-www-form-urlencoded",
                // method: "post",
                // prevent_default: "onsubmit",
                onsubmit: move |evt| {
                    info!("submitting survey result: {:?}", evt.values);
                    submit_survey(
                        evt,
                        survey_id.clone().to_owned(),
                        app_state.read().user.as_ref().unwrap().token.clone().to_owned(),
                    )
                },
                onchange: move |evt| {
                    info!("form: {:#?}", evt.data);
                },
                if survey_state.value().is_some() {
                    let survey = survey_state.value().unwrap().as_ref().unwrap();
                    let rendered_questions = survey.questions.iter().map(|question| {
                        info!("curr question: {:?}" ,question);
                        // let curr_state = answer_state.get().get(&question.id.clone()).unwrap();
                        rsx!{
                            Question {
                                question: question,
                            }
                        }}
                    );

                    rsx!{
                        h1 { "title: {survey.title:?}" }
                        {rendered_questions}
                    }
                }
                button {
                    class: "",
                    // onsubmit: move |evt| submit_survey(
                    //     evt,
                    //     survey_id.clone().to_owned(),
                    //     app_state.read().user.as_ref().unwrap().token.clone().to_owned(),
                    // ),
                    r#type: "submit",
                    "Submit response"
                }
            }
        }
    }
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
