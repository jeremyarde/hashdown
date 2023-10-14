use dioxus::prelude::*;
use dioxus::{html::button, prelude::*};

use log::info;
use serde::Serialize;
use serde_json::{json, Value};

use crate::mainapp::{AppError, AppState, LoginPayload, UserContext};

#[derive(Serialize, Debug)]
pub struct SignupPayload {
    first_name: String,
    // last_name: String,
    // phone_number: String,
    email_address: String,
    password: String,
}

#[component]
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
        rsx!{
                div {
                    form {
                        onsubmit: move |evt| onsubmit(evt, app_state.read().client.clone()),
                        class: "login-form",
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
            },

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

#[component]
pub fn Signup(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let show_login = use_state(cx, move || false);
    let show_signup = use_state(cx, move || false);

    // let myclient = use_atom_state(cx, CLIENT);

    let onsubmit = move |evt: FormEvent, client: reqwest::Client| {
        cx.spawn({
            to_owned![app_state];

            async move {
                let url = "http://localhost:3000/auth/signup";
                let request = SignupPayload {
                    email_address: evt.values["email"].get(0).unwrap().to_owned(),
                    password: evt.values["password"].get(0).unwrap().to_owned(),
                    first_name: evt.values["first_name"].get(0).unwrap().to_owned(),
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
                        println!("Signup successful!");
                        let response: Value = data.json().await.expect("Login data was not json");
                        match response.get("auth_token") {
                            Some(x) => {
                                info!("Logged in successfully");
                                info!("REMOVE ME: token {x}");
                                app_state.write().user = Some(UserContext {
                                    username: request.email_address,
                                    token: x.as_str().unwrap().to_string(),
                                    cookie: "".to_string(),
                                });
                            }
                            None => {
                                info!("Did not log in, could not find auth token");
                                app_state.write().user = None;
                            }
                        }
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

    render! {

        div {
            form {
                onsubmit: move |evt| onsubmit(evt, app_state.read().client.clone()),
                class: "signup-form",
                input { r#type: "text", id: "email", name: "email" }
                label { "email" }
                br {}
                input { r#type: "password", id: "password", name: "password" }
                label { "Password" }
                br {}
                button { "Login" }
            }
        }
    }
}
