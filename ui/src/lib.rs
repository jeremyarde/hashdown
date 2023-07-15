#![allow(non_snake_case)]

mod pages;
use pages::login::Login;
use pages::survey::RenderSurvey;

// #![feature(async_closure)]
pub mod mainapp {
    use std::{
        collections::HashMap,
        error,
        str::FromStr,
        time::{self, Instant},
    };

    use dioxus_router::{Link, Redirect, Route, Router};
    use gloo_timers::{callback::Timeout, future::TimeoutFuture};
    // use console_log::log;
    use log::info;
    // use db::database::Database;

    // use std::{thread::sleep, time::Duration};

    use dioxus::{
        html::{button, fieldset, legend, style},
        prelude::*,
    };
    use fermi::{use_atom_ref, use_atom_state, Atom, AtomRef, AtomRoot};

    use fermi::use_init_atom_root;
    use serde_json::{json, Value};

    use crate::pages::{login::Login, survey::RenderSurvey};

    // use dioxus_router::{Link, Route, Router};
    // use dioxus_router::{Link, Route, Router};
    // use dioxus_ssr::render_lazy;
    // use gloo_timers::future::TimeoutFuture;
    // use gloo_timers::future::TimeoutFuture;
    // use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
    use markdownparser::{
        parse_markdown_v3, ParsedSurvey, Question, QuestionOption, QuestionType, Questions, Survey,
    };

    // mod types;
    // use types::SurveyDto;

    // use gloo_timers::future::TimeoutFuture;
    use reqwest::{header, Client, RequestBuilder};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    struct CreateSurvey {
        plaintext: String,
    }

    #[derive(Serialize, Debug, Clone)]
    pub struct UserContext {
        pub username: String,
        pub token: String,
        pub cookie: String,
    }

    impl UserContext {
        fn new() -> Self {
            return Self {
                username: "jeremy".to_string(),
                token: "".to_string(),
                cookie: "".to_string(),
            };
        }

        fn from(token: String) -> Self {
            return Self {
                username: "jeremy".to_string(),
                token: token.trim().replace("\"", "").to_owned(),
                cookie: "".to_string(),
            };
        }
    }

    #[derive(Debug)]
    pub enum AppError {
        NotLoggedIn,
        Idle,
    }

    #[derive(Debug)]
    pub struct AppState {
        // questions: Questions,
        pub input_text: String,
        pub client: Client,
        // surveys: Vec<Survey>,
        // pub surveys: Vec<SurveyDto>,
        // pub curr_survey: SurveyDto,
        pub user: Option<UserContext>,
        // auth_token: String,
        pub show_login: bool,
        pub survey: Survey,
        pub state: AppError,
    }

    impl AppState {
        fn set_user(&mut self, user: UserContext) {
            self.user = Some(user);
        }
    }

    impl AppState {
        fn new() -> Self {
            // let db = Database::new(false).;

            let mut headers = header::HeaderMap::new();
            // headers.insert(
            //     // "Content-Type",
            //     header::CONTENT_TYPE,
            //     header::HeaderValue::from_static("application/json"),
            // );
            // headers.insert(header::CONTENT_ENCODING,
            // header::)

            let client = reqwest::Client::builder()
                .default_headers(headers)
                // .cookie_store(true) // does not work on wasm right now: https://github.com/seanmonstar/reqwest/pull/1753
                .build()
                .unwrap();
            AppState {
                // questions: Questions { qs: vec![] },
                input_text: String::from(""),
                client: client,
                user: None,
                survey: Survey::new(),
                show_login: false,
                state: AppError::NotLoggedIn,
            }
        }
    }

    // impl SuveyDto {
    //     fn new() -> SurveyDto {}
    // }

    pub static APP: AtomRef<AppState> = |_| AppState::new();
    // static CLIENT: Atom<reqwest::Client> = |_| reqwest::Client::new();
    static EDITOR: Atom<String> = |_| String::from("");
    static REQ_TIMEOUT: Atom<TimeoutFuture> = |_| TimeoutFuture::new(2000);

    const FORMINPUT_KEY: &str = "forminput";

    fn Editor(cx: Scope) -> Element {
        let editor_state = use_atom_state(&cx, EDITOR);
        let toast_visible = use_atom_state(&cx, TOAST);
        // let question_state = use_atom_state(&cx, APP);
        let app_state = use_atom_ref(&cx, APP);
        // let send_request_timeout = use_atom_state(&cx, REQ_TIMEOUT);
        let send_req_timeout = use_atom_state(&cx, REQ_TIMEOUT);
        let some_timeout = use_state(&cx, || TimeoutFuture::new(2000));
        let create_survey = move |content: String, client: Client| {
            cx.spawn({
                to_owned![editor_state, app_state, send_req_timeout];
                if app_state.read().user.is_none() {
                    info!("Not logged in yet.");
                    return;
                }
                // timeout.get()
                async move {
                    let something = send_req_timeout.get();
                    let token = app_state.read().user.clone().unwrap().token;
                    // timeout.await;
                    // TimeoutFuture::new(2000).await;
                    // let t = Timeou
                    info!("Attempting to save questions...");
                    // info!("Questions save: {:?}", question_state);
                    match app_state
                        .read()
                        .client
                        .post("http://localhost:3000/surveys")
                        .json(&CreateSurvey {
                            plaintext: editor_state.get().clone(),
                        })
                        // .bearer_auth(token)
                        .send()
                        .await
                    {
                        Ok(x) => {
                            info!("success: {x:?}");
                            // match SurveyDto::from(content.clone()) {
                            //     Ok(sur) => {
                            //         app_state.modify(|curr| {
                            //             AppState {
                            //                 // questions: Questions { qs: vec![] },
                            //                 input_text: curr.input_text.clone(),
                            //                 client: curr.client.clone(),
                            //                 // surveys: vec![],
                            //                 // curr_survey: sur,
                            //                 user: curr.user.to_owned(),
                            //                 show_login: curr.show_login,
                            //                 survey: curr.survey.to_owned(),
                            //                 state: todo!(),
                            //                 // auth_token: curr.auth_token.clone(),
                            //             }
                            //             // curr.questions = question;
                            //         });
                            //         // let _x = &set_app.get().questions;
                            //         editor_state.set(content);
                            //         // info!("should show toast now");
                            //         // toast_visible.set(true);
                            //     }
                            //     Err(_) => {}
                            // }
                        }
                        Err(x) => info!("error: {x:?}"),
                    }

                    // timeout = Timeout::new(1000, callback)
                }
            })
        };

        let post_questions = move |content| {
            cx.spawn({
                to_owned![toast_visible, app_state];

                if app_state.read().user.is_none() {
                    info!("user token is not set");
                    return;
                }

                let mut token = app_state.read().user.clone().unwrap().token;
                token = token.trim_matches('"').to_string();
                async move {
                    info!("Attempting to save questions...");
                    info!("Publishing content, app_state: {:?}", app_state.read());
                    // info!("Questions save: {:?}", question_state);
                    match app_state
                        .read()
                        .client
                        .post("http://localhost:3000/surveys")
                        .json(&CreateSurvey { plaintext: content })
                        // .bearer_auth(token.clone())
                        .header("x-auth-token", token)
                        .send()
                        .await
                    {
                        Ok(x) => {
                            info!("success: {x:?}");
                            info!("should show toast now");
                            toast_visible.set(true);
                        }
                        Err(x) => info!("error: {x:?}"),
                    };
                }
            })
        };

        let editor_survey = move |content: String| {
            let survey = match ParsedSurvey::from(content) {
                Ok(x) => {
                    info!("Parsed: {x:#?}");
                    app_state.write().survey = Survey::from(x);
                    // app_state.write().modify(|curr| {
                    //     AppState {
                    //         input_text: curr.input_text.clone(),
                    //         client: curr.client.clone(),
                    //         // surveys: vec![],
                    //         // curr_survey: curr.curr_survey.to_owned(),
                    //         user: curr.user.to_owned(),
                    //         show_login: curr.show_login,
                    //         survey: Survey::from(x),
                    //         state: AppError::Idle,
                    //     }
                    // });
                }
                Err(_) => {}
            };
        };

        cx.render(rsx! {
            div { class: "editor-container",
                form {
                    class: "editor-form",
                    prevent_default: "onsubmit",
                    // action: "localhost:3000/survey",
                    onsubmit: move |evt| {
                        info!("Pushed publish :)");
                        let formvalue = evt.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                        post_questions(formvalue);
                        evt.stop_propagation();
                    },
                    // oninput: move |e| {
                    //     let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                    //     editor_state.set(formvalue);
                    // },

                    oninput: move |e| {
                        let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                        editor_survey(formvalue.clone());
                        info!("onchange results: {:?}", formvalue);
                    },
                    textarea {
                        class: "editor-field",
                        required: "",
                        rows: "8",
                        placeholder: "Write your survey here",
                        name: FORMINPUT_KEY
                    }

                    button { class: "publish-button",
                        // r#type: "submit",
                        "Publish"
                    }
                }
            }
        })
    }

    static TOAST: Atom<bool> = |_| false;

    fn Toast(cx: Scope) -> Element {
        let toast_visible = use_atom_state(&cx, TOAST);

        let timer = async move {
            cx.spawn({
                to_owned![toast_visible];
                // TimeoutFuture::new(1_000).await;
                // toast_visible.set(false);
                async move {
                    // Timeout::new(2000, move || {
                    //     toast_visible.set(false);
                    // })
                    // .forget();
                    TimeoutFuture::new(1_000).await;
                    toast_visible.set(false);
                }
            })
        };

        cx.render(rsx! {
            toast_visible.then(|| {
            cx.spawn({
                to_owned![toast_visible];
                // TimeoutFuture::new(1_000).await;
                // toast_visible.set(false);
                async move {
                    // Timeout::new(2000, move || {
                    //     toast_visible.set(false);
                    // })
                    // .forget();
                    info!("before timeout");
                    // TimeoutFuture::new(7000).await;
                    toast_visible.set(false);
                    info!("after timeout");
                }
            });
            rsx!{
                div {
                    onclick: move |_| {
                        toast_visible.set(false)
                    },
                    class:"fixed right-10 bottom-10 px-5 py-4 border-r-8 bg-white drop-shadow-lg fade-in transition ease-in-out hover:-translate-y-1 hover:scale-110 hover:bg-indigo-500 duration-1000 from-blue-500",
                    p {
                        span {
                            class: "mr-2 inline-block px-3 py-1 rounded-full bg-blue-500 text-white font-extrabold",
                            "i"
                        }
                        "Successfully created the survey!"
                    }
                }
            }
        })
        })
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct LoginPayload {
        pub email: String,
        pub password: String,
    }

    pub fn Navbar(cx: Scope) -> Element {
        let app_state = use_atom_ref(&cx, APP);

        let signup = move |authmethod: String| {
            cx.spawn({
                to_owned![app_state];
                async move {
                    info!("Attempting signup...");
                    // info!("Questions save: {:?}", question_state);

                    match app_state
                        .read()
                        .client
                        .post(format!("http://localhost:3000/{authmethod}"))
                        .json(&LoginPayload {
                            email: "a@a.a".to_string(),
                            password: "a".to_string(),
                        })
                        .send()
                        .await
                    {
                        Ok(x) => {
                            info!("signup success: {x:?}");
                            let token = x
                                .json::<Value>()
                                .await
                                .expect("Could not deserialize signup result");
                            let token_text = token
                                .get("auth_token")
                                .expect("Did not find auth_token in signup result");
                            let new_user = UserContext::from(token_text.to_string());
                            info!("new user context: {token} {token_text} {new_user:?}");

                            // app_state.modify(|curr| AppState {
                            //     input_text: curr.input_text.clone(),
                            //     client: curr.client.clone(),
                            //     // surveys: curr.surveys.to_owned(),
                            //     // curr_survey: curr.curr_survey.clone(),
                            //     user: Some(new_user.to_owned()),
                            //     show_login: curr.show_login,
                            //     survey: curr.survey.to_owned(),
                            //     state: AppError::Idle,
                            // });
                            app_state.write().user = Some(new_user);
                        }
                        Err(x) => info!("error: {x:?}"),
                    };
                }
            })
        };

        cx.render(rsx! {
            div { class: "navbar",
                div { style: "", a { href: "/", class: "navbar-home", "Navbar here" } }
                div { style: "",
                    button {
                        class: "navbar-login",
                        // onclick: move |e| {signup()}
                        onclick: move |evt| {
                            info!("Pushed login :)");
                            evt.stop_propagation();
                        },
                        "login"
                    }
                    button {
                        class: "navbar-signup",
                        onclick: move |evt| {
                            info!("Pushed publish :)");
                            signup("signup".to_string());
                            evt.stop_propagation();
                        },
                        "signup"
                    }
                }
            }
        })
    }

    fn NotFound(cx: Scope) -> Element {
        cx.render(rsx!( div { "YO THIS IS NOT FOUND" } ))
    }

    pub fn App(cx: Scope) -> Element {
        use_init_atom_root(cx);
        let app_state = use_atom_ref(cx, APP);
        let editor_state = use_atom_state(cx, EDITOR);

        cx.render(rsx!(
            main {
                // class: "container mx-auto max-w-lg p-6",
                class: "container p-6",
                div {
                    self::Navbar {}
                    div { class: "editor-view",
                        div { style: "grid-column:1", self::Editor {} }
                        div { style: "grid-column:2", RenderSurvey { survey_to_render: &app_state.read().survey.survey } }
                    }
                    Login {}
                }
            }
        ))
    }
}
