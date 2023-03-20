#![allow(non_snake_case)]
// #![feature(async_closure)]
pub mod mainapp {

    // use std::{thread::sleep, time::Duration};

    use dioxus::prelude::*;

    // use dioxus_router::{Link, Route, Router};
    use dioxus_router::{Link, Route, Router};
    use dioxus_ssr::render_lazy;
    use fermi::{use_atom_state, use_read, Atom, AtomRoot};
    // use gloo_timers::future::TimeoutFuture;
    // use gloo_timers::future::TimeoutFuture;
    // use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
    // use markdownparser::{
    //     nanoid_gen, parse_markdown_blocks, parse_markdown_v3, Question, QuestionType, Questions,
    // };

    // mod types;
    // use types::SurveyDto;

    use gloo_timers::future::TimeoutFuture;
    use reqwest::{header, Client};
    use serde::{Deserialize, Serialize};

    static APP: Atom<AppState> = |_| AppState::new();

    #[derive(Serialize)]
    struct CreateSurvey(String);

    #[derive(Debug)]
    struct AppState {
        // questions: Questions,
        input_text: String,
        client: Client,
        // surveys: Vec<Survey>,
        surveys: Vec<SurveyDto>,
        curr_survey: SurveyDto,
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
    struct SurveyDto {
        id: String,
        nanoid: String,
        plaintext: String,
        user_id: String,
        created_at: String,
        modified_at: String,
        version: String,
        // questions: Questions,
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
                // questions: Questions { qs: vec![] },
            }
        }
        fn from(text: String) -> SurveyDto {
            SurveyDto {
                id: "".to_string(),
                nanoid: "".to_string(),
                plaintext: text,
                user_id: "".to_string(),
                created_at: "".to_string(),
                modified_at: "".to_string(),
                version: "".to_string(),
                // questions: parse_markdown_v3(text).unwrap(),
            }
        }
    }

    impl AppState {
        fn new() -> Self {
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
                .build()
                .unwrap();
            AppState {
                // questions: Questions { qs: vec![] },
                input_text: String::from(""),
                client: client,
                surveys: vec![],
                // curr_survey: Survey::new(),
                curr_survey: SurveyDto {
                    id: "".to_string(),
                    nanoid: "".to_string(),
                    plaintext: "".to_string(),
                    user_id: "".to_string(),
                    created_at: "".to_string(),
                    modified_at: "".to_string(),
                    version: "".to_string(),
                },
            }
        }
    }

    // impl SuveyDto {
    //     fn new() -> SurveyDto {}
    // }

    static EDITOR: Atom<String> = |_| String::from("");
    // static FORMINPUT_KEY: Atom<String> = |_| String::from("forminput");
    const FORMINPUT_KEY: &str = "forminput";

    fn Editor(cx: Scope) -> Element {
        let editor_state = use_atom_state(&cx, EDITOR);
        let question_state = use_atom_state(&cx, APP);

        let send_input = move |content: String| {
            // println!("Recieved input: {content}");
            // let question = parse_markdown_blocks(content.clone());
            // let question = parse_markdown_v3(content.clone());
            // println!("Questions: {question:#?}");

            question_state.modify(|curr| {
                AppState {
                    // questions: Questions { qs: vec![] },
                    input_text: curr.input_text.clone(),
                    client: curr.client.clone(),
                    surveys: vec![],
                    curr_survey: SurveyDto::from(content.clone()),
                }
                // curr.questions = question;
            });
            // let _x = &set_app.get().questions;
            editor_state.set(content);
        };

        cx.render(rsx! {
        div{
            form {
                prevent_default: "onclick",
                oninput: move |e| {
                    println!("form event: {e:#?}");
                    let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                    send_input(formvalue);
                },
                div { class: "p-4 rounded-xl bg-white dark:bg-gray-800 focus:ring-red-500",
                    id: "editor",
                    label { class: "sr-only",
                        r#for: "editor",
                        "Publish post"
                    }
                    textarea { class: " w-full resize-y rounded-xl text-sm text-gray-800 bg-white border-0 dark:bg-gray-800 dark:text-white dark:placeholder-gray-400",
                        required: "",
                        rows: "8",
                        placeholder: "Write your survey here",
                        name: "forminput"
                        // oninput: move |e| {send_input(e.value.clone())},
                    }
                    Publish {}
                }
            }
        }
    })
    }

    fn Publish(cx: Scope) -> Element {
        // let question_state = use_atom_state(&cx, APP);
        let app_state = use_atom_state(&cx, APP);
        let toast_visible = use_atom_state(&cx, TOAST);

        // let post_questions = move || {
        //     println!("Attempting to save questions...");
        //     println!("Questions save: {:?}", question_state);
        //     app_state
        //         .client
        //         .post("localhost:3000/survey")
        //         .body(question_state)
        //         .send()
        //         .await?;
        // };

        let post_questions = move |content, client: Client| {
            cx.spawn({
                to_owned![toast_visible];
                async move {
                    println!("Attempting to save questions...");
                    // println!("Questions save: {:?}", question_state);
                    match client
                        .post("http://localhost:3000/survey")
                        .json(&CreateSurvey(content))
                        .send()
                        .await
                    {
                        Ok(x) => {
                            // info!("success: {x:?}");
                            println!("should show toast now");
                            toast_visible.set(true);
                        }
                        Err(x) => println!("error: {x:?}"),
                    };
                }
            })
        };

        cx.render(rsx! {
            button {
                prevent_default: "onclick",
                class: "hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2",
                onclick: move |evt| {
                    println!("Pushed publish :)");
                    post_questions("test".to_string(), app_state.client.clone());
                    evt.stop_propagation();
                },
                "Publish"
            }
        })
    }

    fn Home(cx: Scope) -> Element {
        let app_state = use_read(cx, APP);
        // let test = SurveyDto::from("- this is a thing".to_string());
        // let test = SurveyComponentProps {
        //     visible: true,
        //     survey: app_state.curr_survey,
        // };
        cx.render(rsx! {
            main{
                // class: "container mx-auto max-w-lg p-6",
                class: "container p-6",
                div {
                    // self::navbar {}
                    // self::ListSurveyButton {},
                    self::Editor {},
                    self::RenderSurvey { survey_to_render: &app_state.curr_survey },
                    // SurveysComponent { survey: &app_state.curr_survey }
                    self::Toast {},
                }
            }
        })
    }

    // fn navbar(cx: Scope) -> Element {
    //     cx.render(rsx! {
    //         ul {
    //             Link { to: "/surveys", "Go to all Surveys" }
    //             br {}
    //             Link { to: "/", "Home"}
    //         }
    //     })
    // }

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
                    // TimeoutFuture::new(1_000).await;
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
                    println!("before timeout");
                    TimeoutFuture::new(7000).await;
                    toast_visible.set(false);
                    println!("after timeout");
                }
            });
            rsx!{
                div {
                    onclick:  move |_| {
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
        // let questions = parse_markdown_v3(survey_to_render.plaintext.clone()).questions;

        // let questions = all_questions.get(0).unwrap();
        cx.render(rsx! {
                // questions.iter().map(|q|
            // div {
            //     class: "outline-1 outline-red-400 bg-blue-600",
            //     form {
            //     prevent_default: "onclick",
            //     questions.iter().map(|q: &Question| rsx!{
            //     // app_state.questions.qs.iter().map(|q: &Question| rsx!{
            //         // surveyspage.get().surveys.into_iter().map(|s| s.questions.into_iter().map(|q| rsx! {
            //         // .iter().map(|q: &Question| rsx!{
            //         fieldset {
            //             legend {
            //                 class: "",
            //                 "{q.value} - {q.r#type:?}"
            //             }
            //             {
            //                 q.options.iter().map(|option| {
            //                     let qtype = match q.r#type {
            //                         QuestionType::Radio => "radio",
            //                         QuestionType::Checkbox => "checkbox",
            //                         QuestionType::Text => "textarea",
            //                     };

            //                     rsx!{
            //                         div{
            //                             key: "{option.id}",
            //                             class: "flex items-center",
            //                             input {
            //                                 id: "{option.id}",
            //                                 name: "{q.id}",
            //                                 r#type: "{qtype}",
            //                                 class: " m-3 border border-gray-400"
            //                             }
            //                             label {
            //                                 r#for: "{option.id}",
            //                                 class: " text-gray-700 font-medium",
            //                                 "{option.text}"
            //                             }
            //                         }
            //                     }
            //                 })
            //             }
            //         }
            //     })
            // }
            "Temporary, this is survey render area"
        })
    }

    // mod mainapp {
    // use super::EDITOR;

    // use super::APP;

    // use fermi::use_atom_state;

    use fermi::use_init_atom_root;
    // use ui::mainapp::app;

    fn ListSurveysComponent(cx: Scope) -> Element {
        let app_state = use_atom_state(&cx, APP);
        println!("In list survey components");
        // let thing = move || {
        //     cx.spawn(async move {
        //         to_owned![app_state];
        //         let surveys = list_surveys(&app_state.client).await;
        //         app_state.modify(|curr| AppState {
        //             questions: curr.questions.clone(),
        //             input_text: curr.input_text.clone(),
        //             client: curr.client.clone(),
        //             surveys: surveys,
        //             curr_survey: curr.curr_survey.clone(),
        //         });
        //     })
        // };

        // let post_questions = move |content, client: Client| {
        //     let id = nanoid_gen(12);
        //     cx.spawn({
        //         to_owned![toast_visible];
        //         async move {
        //             println!("Attempting to save questions...");
        //             // println!("Questions save: {:?}", question_state);
        //             match client
        //                 .post("http://localhost:3000/survey")
        //                 .json(&CreateSurvey {
        //                     id,
        //                     plaintext: content,
        //                 })
        //                 .send()
        //                 .await
        //             {
        //                 Ok(x) => {
        //                     println!("success: {x:?}");
        //                     println!("should show toast now");
        //                     toast_visible.set(true);
        //                 }
        //                 Err(x) => println!("error: {x:?}"),
        //             };
        //         }
        //     })
        // };

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
                    });
                }
            })
        };

        println!("list survey component");

        cx.render(rsx! {
            div {
                class: "bg-green-400",
                h1 {
                    "All Surveys"
                }
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
                println!("successfully listing surveys: {x:?}");
                return x
                    .json::<Vec<SurveyDto>>()
                    .await
                    .expect("Could not parse json surveys");
            }
            Err(x) => {
                println!("error listing surveys: {x:?}");
                return vec![];
            }
        }
    }

    pub fn App(cx: Scope) -> Element {
        use_init_atom_root(cx);

        let set_app = use_atom_state(cx, APP);
        let editor_state = use_atom_state(cx, EDITOR);

        cx.render(rsx!(
            // Route { to: "/", Home {}}
                Router {
                    // ul {
                    //     Link { to: "/" li {"home"}}
                    //     Link {to: "/surveys", li {"list surveys"}}
                    // }
                    // Route { to: "", self::Home {}},
                    // Route {
                    //     to: "/releases",
                    //     Releases { },
                    // },
                    Route { to: "", Home {}}
                },
            // Home {}
            // Editor {}
        ))
    }

    pub fn server_side() -> String {
        dioxus_ssr::render_lazy(rsx! {
            // div{"test"},
            App {}

        })
    }
}
