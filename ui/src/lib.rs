#![allow(non_snake_case)]

mod pages;
use pages::login::Login;

// #![feature(async_closure)]
pub mod mainapp {
    use std::{time::{self, Instant}, error};

    use dioxus_router::{Router, Route, Link, Redirect};
    use gloo_timers::{callback::Timeout, future::TimeoutFuture};
    // use console_log::log;
    use log::info;
    // use db::database::Database;

    // use std::{thread::sleep, time::Duration};

    use dioxus::{
        html::{button, style, fieldset, legend},
        prelude::*,
    };
    use fermi::{use_atom_state, Atom, AtomRoot};

    // use dioxus_router::{Link, Route, Router};
    // use dioxus_router::{Link, Route, Router};
    // use dioxus_ssr::render_lazy;
    // use gloo_timers::future::TimeoutFuture;
    // use gloo_timers::future::TimeoutFuture;
    // use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
    use markdownparser::{
        parse_markdown_blocks, parse_markdown_v3, Question, QuestionType, Questions, QuestionOption,
    };

    // mod types;
    // use types::SurveyDto;

    // use gloo_timers::future::TimeoutFuture;
    use reqwest::{header, Client, RequestBuilder};
    use serde::{Deserialize, Serialize};



    pub static APP: Atom<AppState> = |_| AppState::new();
    pub static CLIENT: Atom<reqwest::Client> = |_| reqwest::Client::new();


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
    pub struct AppState {
        // questions: Questions,
        pub input_text: String,
        pub client: Client,
        // surveys: Vec<Survey>,
        pub surveys: Vec<SurveyDto>,
        pub curr_survey: SurveyDto,
        pub user: Option<UserContext>,
        // auth_token: String,
        pub show_login: bool,
    }

    impl AppState {
        fn set_user(&mut self, user: UserContext) {
            self.user = Some(user);
        }
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    struct Survey {
        id: String,
        nanoid: String,
        plaintext: String,
        user_id: String,
        created_at: String,
        modified_at: String,
        version: String,
        // questions: Questions,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct SurveyDto {
        id: String,
        nanoid: String,
        plaintext: String,
        user_id: String,
        created_at: String,
        modified_at: String,
        version: String,
        questions: Vec<Question>,
    }

    impl SurveyDto {
        fn new() -> SurveyDto {
            SurveyDto {
                id: "".to_string(),
                nanoid: "".to_string(),
                plaintext: "".to_string(),
                user_id: "".to_string(),
                created_at: "".to_string(),
                modified_at: "".to_string(),
                version: "".to_string(),
                questions: vec![],
            }
        }
        fn from(text: String) -> anyhow::Result<SurveyDto>{
            let survey = parse_markdown_v3(text)?;
            return Ok(SurveyDto {
                id: "".to_string(),
                nanoid: "".to_string(),
                plaintext: survey.plaintext,
                user_id: "".to_string(),
                created_at: "".to_string(),
                modified_at: "".to_string(),
                version: survey.parse_version,
                questions: survey.questions,
            });
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
                surveys: vec![],
                // auth_token: "".to_string(),
                // curr_survey: Survey::new(),
                curr_survey: SurveyDto {
                    id: "".to_string(),
                    nanoid: "".to_string(),
                    plaintext: "".to_string(),
                    user_id: "".to_string(),
                    created_at: "".to_string(),
                    modified_at: "".to_string(),
                    version: "".to_string(),
                    questions: vec![],
                },
                user: None,
                show_login: false,
            }
        }
    }

    // impl SuveyDto {
    //     fn new() -> SurveyDto {}
    // }

    static EDITOR: Atom<String> = |_| String::from("");
    static REQ_TIMEOUT: Atom<TimeoutFuture> = |_| TimeoutFuture::new(2000);
    // static FORMINPUT_KEY: Atom<String> = |_| String::from("forminput");
    const FORMINPUT_KEY: &str = "forminput";

    fn Editor(cx: Scope) -> Element {
        let editor_state = use_atom_state(&cx, EDITOR);
        let toast_visible = use_atom_state(&cx, TOAST);
        // let question_state = use_atom_state(&cx, APP);
        let app_state = use_atom_state(&cx, APP);
        // let send_request_timeout = use_atom_state(&cx, REQ_TIMEOUT);
        let send_req_timeout = use_atom_state(&cx, REQ_TIMEOUT);
        let some_timeout = use_state(&cx, || TimeoutFuture::new(2000));

        let create_survey = move |content: String, client: Client| {
            cx.spawn({
                to_owned![editor_state, app_state, send_req_timeout];
                if app_state.user.is_none() {
                    info!("Not logged in yet.");
                    return;
                }
                // timeout.get()
                async move {
                    let something = send_req_timeout.get();
                    let token  = app_state.user.clone().unwrap().token;
                    // timeout.await;
                    // TimeoutFuture::new(2000).await;
                    // let t = Timeou
                    info!("Attempting to save questions...");
                    // info!("Questions save: {:?}", question_state);
                    match app_state.client
                        .post("http://localhost:3000/surveys")
                        .json(&CreateSurvey {
                            plaintext: editor_state.get().clone(),
                        })
                        .bearer_auth(token)
                        .send()
                        .await
                    {
                        Ok(x) => {
                            info!("success: {x:?}");
                            match SurveyDto::from(content.clone()) {
                                Ok(sur) => {
                                    app_state.modify(|curr| {
                                        AppState {
                                            // questions: Questions { qs: vec![] },
                                            input_text: curr.input_text.clone(),
                                            client: curr.client.clone(),
                                            surveys: vec![],
                                            curr_survey: sur,
                                            user: curr.user.to_owned(),
                                            show_login: curr.show_login,
                                            // auth_token: curr.auth_token.clone(),
                                        }
                                        // curr.questions = question;
                                    });
                                    // let _x = &set_app.get().questions;
                                    editor_state.set(content);
                                    // info!("should show toast now");
                                    // toast_visible.set(true);
                                }
                                Err(_) => {}
                            }
                        }
                        Err(x) => info!("error: {x:?}"),
                    }

                    // timeout = Timeout::new(1000, callback)
                }
            })
        };


        let post_questions = move |content, client: Client| {
            cx.spawn({
                to_owned![toast_visible, app_state];

                if app_state.user.is_none() {
                    info!("user token is not set");
                }
                let mut token  = app_state.user.clone().unwrap().token;
                token = token.trim_matches('"').to_string();
                async move {
                    info!("Attempting to save questions...");
                    info!("Publishing content, app_state: {app_state:?}");
                    // info!("Questions save: {:?}", question_state);
                    match client
                        .post("http://localhost:3000/surveys")
                        .json(&CreateSurvey { plaintext: content })
                        .bearer_auth(token.clone())
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

        cx.render(rsx! {
            div {
                class: "editor-container",
                form {
                    class: "editor-form",
                    prevent_default: "onsubmit",
                    // action: "localhost:3000/survey",
                    onsubmit: move |evt| {
                        // evt.prevent_default();
                            info!("Pushed publish :)");
                            let formvalue = evt.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                            post_questions(formvalue, app_state.client.clone());
                            evt.stop_propagation();
                    },
                    // oninput: move |e| {
                    //     let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                    //     editor_state.set(formvalue);
                    // },
                    
                    oninput: move |e| {
                        let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                        // let formvalue = "- this is a test\n  - this is a question".to_string();
                        match SurveyDto::from(formvalue.clone()) {
                            Ok(sur) => {
                                app_state.modify(|curr| {
                                    AppState {
                                        input_text: curr.input_text.clone(),
                                        client: curr.client.clone(),
                                        surveys: vec![],
                                        curr_survey: sur,
                                        user: curr.user.to_owned(),
                                        show_login: curr.show_login
                                    }
                                });
                            }
                            Err(_) => {}
                        };
                        info!("onchange results: {:?}", formvalue);
                    },
                    textarea {
                        class: "editor-field",
                        required: "",
                        rows: "8",
                        placeholder: "Write your survey here",
                        name: FORMINPUT_KEY,
                    }
                    // button {
                    //     prevent_default: "onclick",
                    //     class: "hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2",
                    //     onclick: move |evt| {
                    //         info!("Pushed publish :)");
                    //         post_questions("test".to_string(), app_state.client.clone());
                    //         evt.stop_propagation();
                    //     },
                    //     "Publish"
                    // }
                    button {
                        class: "publish-button",
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

    #[inline_props]
    fn RenderSurvey<'a>(cx: Scope, survey_to_render: &'a SurveyDto) -> Element {
        let app_state = use_atom_state(cx, APP);

        // let questions = parse_markdown_v3(survey_to_render.plaintext.clone()).questions;
        // let questions = all_questions.get(0).unwrap();
        // let questions: Vec<Question> = vec![];
        // let survey_html
        let curr_survey = app_state.curr_survey.clone();
        cx.render(rsx! {
                div {
                    class: "survey",
                    form {
                        h1 {"form title"}
                        app_state.curr_survey.questions.iter().map(|question| rsx!{
                            fieldset {
                                legend {
                                    "question text: {question.value}"
                                }
                                ul {
                                    question.options.iter().enumerate().map(|(i, option):(usize, &QuestionOption)|
                                    rsx!{
                                        li {
                                            input {
                                                r#type: if question.r#type == QuestionType::Checkbox { "checkbox"} else {"radio"},
                                                value: "o.text: {option.text:?}",
                                                id: "{option.id}_{i}",
                                                name: "{question.id}",
                                            }
                                            label {
                                                r#for:"{option.id}_{i}",
                                                "o.text: {option.text:?}"
                                            }
                                            "{option:?}"
                                        }
                                    })
                                }
                            }
                            
                        })
                    }
                }
        })
    }

    use fermi::use_init_atom_root;
    use serde_json::Value;

    use crate::pages::{self, login::Login};

    fn ListSurveysComponent(cx: Scope) -> Element {
        let app_state = use_atom_state(&cx, APP);
        info!("In list survey components");

        let get_questions = move |client: Client| {
            cx.spawn({
                to_owned![app_state];
                async move {
                    let surveys = list_surveys(&client).await;
                    app_state.modify(|curr| AppState {
                        // questions: curr.questions.clone(),
                        input_text: curr.input_text.clone(),
                        client: curr.client.clone(),
                        surveys: surveys,
                        curr_survey: curr.curr_survey.clone(),
                        user: curr.user.to_owned(),
                        // auth_token: curr.auth_token.clone(),
                        show_login: curr.show_login,
                    });
                }
            })
        };

        info!("list survey component");

        cx.render(rsx! {
            div { class: "bg-green-400",
                h1 { "All Surveys" }
                app_state.surveys.iter().map(|sur| rsx!{
                    h3 {
                        "{sur.nanoid}"
                    }
                })
            }
        })
    }

    async fn list_surveys(client: &Client) -> Vec<SurveyDto> {
        match client.get("http://localhost:3000/survey").send().await {
            Ok(x) => {
                info!("successfully listing surveys: {x:?}");
                return x
                    .json::<Vec<SurveyDto>>()
                    .await
                    .expect("Could not parse json surveys");
            }
            Err(x) => {
                info!("error listing surveys: {x:?}");
                return vec![];
            }
        }
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct LoginPayload {
        pub email: String,
        pub password: String,
    }

    pub fn Navbar(cx: Scope) -> Element {
        let app_state = use_atom_state(&cx, APP);
        let signup = move |authmethod: String, client: Client| {
            cx.spawn({
                to_owned![app_state];
                async move {
                    info!("Attempting signup...");
                    // info!("Questions save: {:?}", question_state);

                    match client
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
                            let token = x.json::<Value>().await.expect("Could not deserialize signup result");
                            let token_text = token.get("auth_token").expect("Did not find auth_token in signup result");
                            let new_user = UserContext::from(token_text.to_string());
                            info!("new user context: {token} {token_text} {new_user:?}");

                            app_state.modify(|curr| AppState {
                                input_text: curr.input_text.clone(),
                                client: curr.client.clone(),
                                surveys: curr.surveys.to_owned(),
                                curr_survey: curr.curr_survey.clone(),
                                user: Some(new_user.to_owned()),
                                show_login: curr.show_login,
                            });

                        }
                        Err(x) => info!("error: {x:?}"),
                    };
                }
            })
        };

        cx.render(rsx! {
            div {
                class: "navbar",
                div {
                    style: "",
                    a { href:"/", class:"navbar-home", "Navbar here"  }
                }
                div {
                    style: "",
                    button {
                        class: "navbar-login",
                        // onclick: move |e| {signup()}
                        onclick: move |evt| {
                            info!("Pushed login :)");
                            // signup("login".to_string(), app_state.client.clone());
                            evt.stop_propagation();
                            app_state.modify(|curr| AppState {
                                input_text: curr.input_text.clone(),
                                client: curr.client.clone(),
                                surveys: curr.surveys.to_owned(),
                                curr_survey: curr.curr_survey.clone(),
                                user: curr.user.to_owned(),
                                show_login: if curr.show_login { false} else { true},
                            });
                        },
                        "login"
                    }
                    button {
                        class: "navbar-signup",
                        onclick: move |evt| {
                            info!("Pushed publish :)");
                            signup("signup".to_string(), app_state.client.clone());
                            evt.stop_propagation();
                        },
                        "signup"
                    }
                }
            }
        })
    }
 
    fn NotFound(cx: Scope) -> Element {
        cx.render(rsx!(
            div {
                "YO THIS IS NOT FOUND"
            }
        ))
    }

    
    // fn Home(cx: Scope) -> Element {
    //     // cx.render(rsx!(
    //     //     div {
    //     //         Router {
    //     //             Route { to: "/", App {}},
    //     //             Route { to: "/login", Login {}}
    //     //             // Route { to: "", NotFound {} }
    //     //             Redirect { from: "", to: "/" }
    //     //         }
    //     //     }
    //     // ))
    // }

    pub fn App(cx: Scope) -> Element {
        use_init_atom_root(cx);
        let app_state = use_atom_state(cx, APP);
        let editor_state = use_atom_state(cx, EDITOR);

        cx.render(rsx!(
            main {
                // class: "container mx-auto max-w-lg p-6",
                class: "container p-6",
                div {
                    self::Navbar {}
                    div {
                        class: "editor-view",
                        div {
                            style: "grid-column:1",
                            self::Editor {}
                        }
                        div {
                            style: "grid-column:2",
                            self::RenderSurvey { survey_to_render: &app_state.curr_survey }
                        }
                    }
                    Login{}
                    // // SurveysComponent { survey: &app_state.curr_survey }
                    // self::Toast {}
                }
            }
        ))
    }
}
