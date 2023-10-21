#![allow(non_snake_case)]

mod pages;

// #![feature(async_closure)]
pub mod mainapp {
    use chrono::{DateTime, Utc};
    use dioxus::{html::EventData, prelude::*};
    use dioxus_router::prelude::*;
    use std::{
        collections::HashMap,
        fmt::Display,
        time::{self, Instant},
    };

    // use gloo_timers::{callback::Timeout, future::TimeoutFuture};
    // use console_log::log;
    use log::info;
    // use db::database::Database;

    // use std::{thread::sleep, time::Duration};

    use dioxus::{
        html::{button, fieldset, legend, style},
        prelude::*,
    };

    use serde_json::{json, Value};

    #[derive(Deserialize, Debug, Serialize)]
    pub struct LoginPayload {
        pub email: String,
        pub password: String,
    }

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

    use crate::pages::auth::Login;
    use crate::pages::auth::Signup;
    use crate::pages::survey::ListSurvey;
    use crate::pages::survey::RenderSurvey;

    #[derive(Serialize)]
    struct CreateSurvey {
        plaintext: String,
    }

    #[derive(Serialize, Debug, Clone, PartialEq)]
    pub struct UserContext {
        pub email: String,
        pub token: String,
        // pub cookie: String,
    }

    impl UserContext {
        fn new() -> Self {
            return Self {
                email: "jeremy".to_string(),
                token: "".to_string(),
                // cookie: "".to_string(),
            };
        }

        fn from(token: String) -> Self {
            return Self {
                email: "jeremy".to_string(),
                token: token.trim().replace("\"", "").to_owned(),
                // cookie: "".to_string(),
            };
        }
    }

    #[derive(Debug, Clone)]
    pub enum AppError {
        NotLoggedIn,
        Idle,
        Generic(String),
    }

    #[derive(Debug)]
    pub struct AppState {
        pub input_text: String,
        pub client: Client,
        pub user: Option<UserContext>,
        pub show_login: bool,
        pub survey: Survey,
        pub state: AppError,
    }

    pub struct Session {
        session_id: String,
        active_period_expires_at: DateTime<Utc>,
        idle_period_expires_at: DateTime<Utc>,
    }

    impl AppState {
        fn set_user(&mut self, user: UserContext) {
            self.user = Some(user);
        }

        // fn validate_session(session_id: Session) -> Session {
        //     // get session from database using existing Session

        //     Session {
        //         session_id: "fake".to_string(),
        //         active_period_expires_at: Utc::now(),
        //         idle_period_expires_at: Utc::now(),
        //     }
        // }
    }

    impl AppState {
        pub fn new() -> Self {
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

        pub fn get_client(&self) {}
    }

    const FORMINPUT_KEY: &str = "forminput";

    #[component]
    fn Editor(cx: Scope) -> Element {
        let editor_state = use_state(&cx, || "".to_string());
        let toast_visible = use_state(&cx, || false);
        let survey_state = use_state(&cx, || Survey::new());

        // let question_state = use_atom_state(&cx, APP);
        // let app_state = use_atom_ref(&cx, &APP);
        let app_state = use_shared_state::<AppState>(cx).unwrap();
        // let send_request_timeout = use_atom_state(&cx, REQ_TIMEOUT);
        // let send_req_timeout = use_atom_state(&cx, &REQ_TIMEOUT);
        let create_survey = move |content: String, client: Client| {
            cx.spawn({
                to_owned![editor_state, app_state];
                if app_state.read().user.is_none() {
                    info!("Not logged in yet.");
                    return;
                }
                // timeout.get()
                async move {
                    // let something = send_req_timeout.get();
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
                    info!("Publishing content,");
                    // info!("Questions save: {:?}", question_state);
                    match app_state
                        .read()
                        .client
                        .post("http://localhost:3000/surveys")
                        .json(&CreateSurvey {
                            plaintext: content.get(0).unwrap().to_owned(),
                        })
                        // .bearer_auth(token.clone())
                        .header("x-auth-token", token)
                        .send()
                        .await
                    {
                        Ok(x) => {
                            info!("success: {x:?}");
                            info!("should show toast now");
                            toast_visible.set(true);
                            app_state.write().state =
                                AppError::Generic("Successfulling saved questions".to_string());
                        }
                        Err(x) => {
                            info!("error: {x:?}");
                            app_state.write().state = AppError::Generic(x.to_string());
                        }
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

    #[component]
    fn Toast(cx: Scope) -> Element {
        let toast_visible = use_state(&cx, || false);

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

    #[component]
    pub fn Navbar(cx: Scope) -> Element {
        // let app_state = use_atom_ref(&cx, &APP);
        let app_state = use_shared_state::<AppState>(cx).unwrap();
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
            h1 { format!("App Errors: {:?}", app_state.read().state) }

            nav { ul {
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
                // li { a { href: "http://localhost:8080/", "home" } }
                // li { a { href: "http://localhost:8080/login", "login" } }
                // li { a { href: "http://ocalhost:8080/signup", "signup" } }
                // li { a { href: "http://localhost:8080/surveys", "surveys" } }
            } }
        })
    }

    #[component]
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

    #[component]
    fn Home(cx: Scope) -> Element {
        let app_state = use_shared_state::<AppState>(cx).unwrap();

        render!( h1 { "Home" } )
    }

    #[component]
    pub fn App(cx: Scope) -> Element {
        // use_init_atom_root(cx);
        use_shared_state_provider(cx, || AppState::new());
        // let app_state = use_atom_ref(cx, &APP);
        // let app_state = use_shared_state::<AppState>(cx).unwrap();

        // let editor_state = use_atom_ref(cx, EDITOR);
        // let editor_state = use_state(cx, || "".to_string());

        // render! { Router::<Route> {} }
        render!(
            div { Navbar {} }
            // div { class: "flex h-screen w-screen items-center justify-center bg-gray-200",
            div { Router::<Route> {} }
        )
    }

    // ANCHOR: router
    #[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Header)]
        #[route("/")]
        Home {},
        #[route("/surveys")] 
        ListSurvey {},
        #[route("/surveys/:survey_id")]
        RenderSurvey { survey_id: String },
        // #[route("signup")]
        // Login {},
        // #[redirect("/signup", || login::Login {})]
        #[route("/login")]
        Login {},

        #[route("/signup")]
        Signup {},
        #[route("/editor")]
        Editor {},
        
        #[route("/:..route")]
        PageNotFound {
            route: Vec<String>,
        },
}
    // ANCHOR_END: router

    #[component]
    fn Header(cx: Scope) -> Element {
        let app_state = use_shared_state::<AppState>(cx).unwrap();

        render! {
            ul {
                li {
                    Link { to: Route::Home {}, "home" }
                }
                if app_state.read().user.is_none() {
                    render!{
                        li {
                            Link { to: Route::Login {}, "login" }
                        }
                        li {
                            Link { to: Route::Signup {}, "signup" }
                        }
                    }
                }

                if app_state.read().user.is_some() {
                    // render !{
                    //     div { class: "", self::Editor {} }
                    //     div { class: "", RenderSurvey { survey_id: "test".to_string() } }
                    // }
                    render !{
                        li {
                            Link { to: Route::Editor  {}, "Editor" }
                        }
                        li {
                            Link { to: Route::ListSurvey {}, "my surveys" }
                        }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }

    #[component]
    fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
        render! {
            h1 { "Page not found" }
            p { "We are terribly sorry, but the page you requested doesn't exist." }
            pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
        }
    }
}
