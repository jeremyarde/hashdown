#![allow(non_snake_case)]

mod pages;
use pages::login::Login;
use pages::survey::RenderSurvey;

// #![feature(async_closure)]
pub mod mainapp {
    use dioxus::prelude::*;
    use dioxus_router::prelude::*;
    use std::{
        collections::HashMap,
        error,
        str::FromStr,
        time::{self, Instant},
    };

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
    use reqwest::{
        header::{self, HeaderValue},
        Client, RequestBuilder,
    };
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
                show_login: true,
                state: AppError::NotLoggedIn,
            }
        }
    }

    // impl SuveyDto {
    //     fn new() -> SurveyDto {}
    // }

    pub static APP: AtomRef<AppState> = AtomRef(|_| AppState::new());
    pub static SURVEY: Atom<Survey> = Atom(|_| Survey::new());
    // static CLIENT: Atom<reqwest::Client> = |_| reqwest::Client::new();
    pub static EDITOR: Atom<String> = Atom(|_| String::from(""));
    static REQ_TIMEOUT: Atom<TimeoutFuture> = Atom(|_| TimeoutFuture::new(2000));

    const FORMINPUT_KEY: &str = "forminput";

    fn Editor(cx: Scope) -> Element {
        let editor_state = use_atom_state(&cx, &EDITOR);
        let toast_visible = use_atom_state(&cx, &TOAST);
        let survey_state = use_atom_state(&cx, &SURVEY);

        // let question_state = use_atom_state(&cx, APP);
        let app_state = use_atom_ref(&cx, &APP);
        // let send_request_timeout = use_atom_state(&cx, REQ_TIMEOUT);
        let send_req_timeout = use_atom_state(&cx, &REQ_TIMEOUT);
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
                        }
                        Err(x) => info!("error: {x:?}"),
                    }

                    // timeout = Timeout::new(1000, callback)
                }
            })
        };

        let post_questions = move |content: Vec<String>| {
            info!("post_questions: {:?}", content);
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
                        .json(&CreateSurvey { plaintext: content.get(0).unwrap().to_owned() })
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

        let editor_survey = move |content: Vec<String>| {
            info!("editor survey content: {:?}", content);
            match ParsedSurvey::from(content.get(0).unwrap().to_owned()) {
                Ok(x) => {
                    info!("Parsed: {x:#?}");
                    app_state.write().survey = Survey::from(x.clone());
                    survey_state.modify(|curr| Survey::from(x));
                }
                Err(_) => {}
            };
        };

        cx.render(rsx! {
            div { class: "w-full h-full",
                form {
                    class: "border border-red-600 flex flex-col",
                    prevent_default: "onsubmit",
                    // action: "localhost:3000/survey",
                    onsubmit: move |evt| {
                        info!("Pushed publish :)");
                        let formvalue = evt.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                        post_questions(formvalue);
                        evt.stop_propagation();
                    },

                    oninput: move |e| {
                        let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                        editor_survey(formvalue.clone());
                        info!("onchange results - editor_state formvalue: {:?}", formvalue);
                        editor_state.modify(|curr| { formvalue.get(0).unwrap().to_owned() });
                    },
                    textarea {
                        class: " bg-transparent resize w-full focus:outline-none border border-emerald-800 focus:border-blue-300",
                        required: "",
                        rows: "8",
                        placeholder: "Write your survey here",
                        name: FORMINPUT_KEY
                    }

                    button { class: "hover:bg-slate-600 transition bg-slate-500",
                        // r#type: "submit",
                        "Publish"
                    }
                }
            }
        })
    }

    static TOAST: Atom<bool> = Atom(|_| false);

    fn Toast(cx: Scope) -> Element {
        let toast_visible = use_atom_state(&cx, &TOAST);

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
                    class: "fixed right-10 bottom-10 px-5 py-4 border-r-8 bg-white drop-shadow-lg fade-in transition ease-in-out hover:-translate-y-1 hover:scale-110 hover:bg-indigo-500 duration-1000 from-blue-500",
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

    pub fn ListSurvey(cx: Scope) -> Element {
        let surveys = use_state(cx, move || vec![]);
        let app_state = use_atom_ref(&cx, &APP);
        let error = use_state(cx, move || "");
        let is_visible = use_state(cx, move || false);

        let onsubmit = move |evt| {
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
                        .get("http://localhost:3000/surveys")
                        .header("x-auth-token", token)
                        .send()
                        .await;

                    match resp {
                        // Parse data from here, such as storing a response token
                        Ok(data) => {
                            info!("successful!");
                            // let jsondata = data.json().await.unwrap();

                            let jsonsurveys = data.json::<Value>().await.unwrap();
                            let surveydata =
                                jsonsurveys.get("surveys").unwrap().as_array().unwrap();
                            surveys.set(surveydata.to_owned());
                        }

                        //Handle any errors from the fetch here
                        Err(_err) => {
                            info!("failed - could not get data.")
                        }
                    }
                }
            });
        };

        let logged_in = app_state.read().user.is_some();

        cx.render(rsx! {
            div {
                if logged_in {
                    rsx!{
                        button {
                            onclick: move |evt| {
                                onsubmit(evt);
                                is_visible.set(true);
                            },
                            "my surveys"
                        }
                        button { 
                            onclick: move |evt| { 
                                if *is_visible.get() { is_visible.set(false) } else { is_visible.set(true) }
                                onsubmit(evt);
                            },
                            if *is_visible.get() {"hide"} else {"show"}
                        }
                    }
                }
            }
            if *is_visible.get() {
                rsx!{
                    div {
                    {if surveys.is_empty() {
                        rsx!(div{"No surveys"})
                    } else {
                        {rsx!(
                            surveys.iter().map(|survey: &Value| {
                            let short = survey.get("id").unwrap();
                            let survey_id = survey.get("survey_id").unwrap().as_str().unwrap();
                            rsx!(
                                div{
                                    "{short:?}",
                                    button {"details"}
                                    a {
                                        href: "http://localhost:3000/surveys/{survey_id}", 
                                        "test"
                                    }
                                }
                            )
                        }))}
                    }}
                }}
        }
        })
    }

    pub fn Navbar(cx: Scope) -> Element {
        let app_state = use_atom_ref(&cx, &APP);

        cx.render(rsx! {
            // div { class: "flex flex-row bg-red-500 p-1 justify-between",
            //     div { class: "justify-start items-start", "Logo HERE" }
            //     div { class: "flex justify-end",
            //         Link { }
            //         button {
            //             class: "bg-green-400",
            //             onclick: move |evt| {
            //                 info!("Pushed publish :)");
            //                 signup("signup".to_string());
            //                 evt.stop_propagation();
            //             },
            //             "signup"
            //         }
            //     }
            // }

            nav {
                ul {
                    // li {
                    //     GoBackButton { "Back" }
                    // }
                    // li {
                    //     Link { to: Route::App {}, "Home" }
                    // }
                    // li {
                    //     Link { to: Route::Login {}, "Login" }
                    // }
                    // li {
                    //     Link { to: Route::ListSurvey {}, "Surveys" }
                    // }

                    li { a { href: "http://localhost:8080/", "home" } }
                    li { a { href: "http://localhost:8080/login", "login" } }
                    li { a { href: "http://ocalhost:8080/signup", "signup" } }
                    li { a { href: "http://localhost:8080/surveys", "surveys" } }
                }
            }
        })
    }

    // #[derive(Routable, Clone)]
    // enum Route {
    //     #[layout(Navbar)]
    //         #[route("/")]
    //         App {},
    //         #[route("/login")]
    //         Login {},
    //         #[route("/surveys")]
    //         ListSurvey {},
    //     #[end_layout]
    //     // #[route("/")]
    //     // App {},
    // }

    pub fn SyntaxExample(cx: Scope) -> Element {
        let example_text = "
# Survey title
- First 
  - multiple choice 1
  - multiple choice 2
  - multiple choice 3
- Second question [checkbox]
  - checkbox 1
  - checkbox 2
        ";
        render! {
            p { style: "white-space: pre-line", example_text }
            p { style: "", example_text }
        }
    }

    pub fn App(cx: Scope) -> Element {
        use_init_atom_root(cx);
        let app_state = use_atom_ref(cx, &APP);
        // let editor_state = use_atom_ref(cx, EDITOR);
        let editor_state = use_state(cx, || "".to_string());

        // render! { Router::<Route> {} }
        cx.render(rsx!(
            div {
                // Navbar {}
                ul {
                    li { Login {} }
                    li { ListSurvey {} }
                }
            }
            div {
            }
            // div { class: "flex h-screen w-screen items-center justify-center bg-gray-200",
            div { class: "",
                div { class: "", self::Editor {} }
                div { class: "", RenderSurvey {} }
                div { class: "", SyntaxExample {} }
            }
        ))
    }
}
