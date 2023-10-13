use dioxus::prelude::*;
use dioxus::{html::button, prelude::*};
use fermi::{use_atom_ref, use_atom_state};
use log::info;
use serde_json::{json, Value};

use crate::mainapp::{AppError, AppState, LoginPayload, UserContext};

pub fn Login(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let show_login = use_state(cx, move || false);
    let show_signup = use_state(cx, move || false);

    // let myclient = use_atom_state(cx, CLIENT);

    let onsubmit = move |evt: FormEvent, client: reqwest::Client| {
        cx.spawn({
            to_owned![app_state];

            async move {
                let url = "http://localhost:3000/auth/login";
                let request: LoginPayload = LoginPayload {
                    email: evt.values["email"].get(0).unwrap().to_owned(),
                    password: evt.values["password"].get(0).unwrap().to_owned(),
                };

                let resp = app_state
                    .read()
                    .client
                    // let resp = reqwest::Client::new()
                    .post(url)
                    .json(&request)
                    .send()
                    .await;

                match resp {
                    // Parse data from here, such as storing a response token
                    Ok(data) => {
                        println!("Login successful!");
                        let response: Value = data.json().await.expect("Login data was not json");
                        match response.get("auth_token") {
                            Some(x) => {
                                info!("Logged in successfully");
                                info!("REMOVE ME: token {x}");
                                app_state.write().user = Some(UserContext {
                                    username: request.email,
                                    token: x.as_str().unwrap().to_string(),
                                    cookie: "".to_string(),
                                });
                            }
                            None => {
                                info!("Did not log in, could not find auth token");
                                app_state.write().user = None;
                            }
                        }

                        // app_state.write().user = app_state.modify(|curr| {
                        //     AppState {
                        //         input_text: curr.input_text.to_owned(),
                        //         client: curr.client.to_owned(),
                        //         state: AppError::Idle,
                        //         // surveys: curr.surveys.to_owned(),
                        //         // curr_survey: curr.curr_survey.to_owned(),
                        //         user: Some(UserContext {
                        //             username: request.email,
                        //             token: response
                        //                 .get("auth_token")
                        //                 .expect("auth_token not found in login")
                        //                 .to_string(),
                        //             cookie: "".to_string(),
                        //         }),
                        //         show_login: curr.show_login,
                        //         survey: curr.survey.to_owned(),
                        //         // auth_token: curr.auth_token.clone(),
                        //     }
                        // });
                    }

                    //Handle any errors from the fetch here
                    Err(_err) => {
                        println!(
                            "Login failed - you need a login server running on localhost:8080."
                        )
                    }
                }
            }
        });
    };

    let onsubmit_signup = move |evt: FormEvent, client: reqwest::Client| {
        cx.spawn({
            to_owned![app_state];
            async move {
                let url = "/auth/signup";
                let client_url = format!("http://{}{}", "localhost:3000", url);

                println!("Sending req to: {client_url}");

                let request: LoginPayload = LoginPayload {
                    email: evt.values["email"].get(0).unwrap().to_owned(),
                    password: evt.values["password"].get(0).unwrap().to_owned(),
                };

                let response = client
                    .post(&client_url)
                    .json(&request)
                    .send()
                    .await
                    .expect("Should recieve response from app");
            }
        });
    };

    cx.render(rsx! {
        button { onclick: move |evt| show_login.modify(|curr| if *curr { false } else { true }),
            "login"
        }
        button { onclick: move |evt| show_signup.modify(|curr| if *curr { false } else { true }),
            "signup"
        }
        if *show_login.get() {
            rsx!{
                div {
                    form {
                        onsubmit: move |evt| onsubmit(evt, app_state.read().client.clone()),
                        class: "login-form",
                        // action: ""
                        // prevent_default: "onsubmit", // Prevent the default behavior of <form> to post
                        input { r#type: "text", id: "email", name: "email" }
                        label { "email" }
                        br {}
                        input { r#type: "password", id: "password", name: "password" }
                        label { "Password" }
                        br {}
                        button {
                            "Login"
                        }

                }}

                // div {
                //     h1 { "Signup" }
                //     form {
                //         onsubmit: move |evt| onsubmit_signup(evt, app_state.client.clone()),
                //         class: "signup-form",
                //         prevent_default: "onsubmit", // Prevent the default behavior of <form> to post
                //         input { r#type: "text", id: "email", name: "email" }
                //         label { "email" }
                //         br {}
                //         input { r#type: "password", id: "password", name: "password" }
                //         label { "Password" }
                //         br {}
                //         button { "Signup" }
                //     }
                // }
            }
        }
        if *show_signup.get() {
            rsx!{
                div {
                    form {
                        onsubmit: move |evt| onsubmit_signup(evt, app_state.read().client.clone()),
                        class: "login-form",
                        prevent_default: "onsubmit", // Prevent the default behavior of <form> to post
                        input { r#type: "text", id: "email", name: "email" }
                        label { "email" }
                        br {}
                        input { r#type: "password", id: "password", name: "password" }
                        label { "Password" }
                        br {}
                        button {
                            "signup"
                        }

                }}
            }
        }
    })
}
