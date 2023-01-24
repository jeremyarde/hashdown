#![allow(non_snake_case)]
use dioxus::{
    events::{onclick, oninput},
    fermi::use_atom_state,
    prelude::*,
};

// use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
use markdownparser::{
    nanoid_gen, parse_markdown_blocks, parse_markdown_v3, Question, QuestionType, Questions,
};
use reqwest::Client;
use serde::Serialize;

static APP: Atom<AppState> = |_| AppState::new();

#[derive(Serialize)]
struct CreateSurvey {
    id: String,
    plaintext: String,
}

#[derive(Debug)]
struct AppState {
    questions: Questions,
    input_text: String,
    client: Client,
}

struct Survey {
    title: String,
    questions: Questions,
}

impl AppState {
    fn new() -> Self {
        let client = reqwest::Client::new();
        AppState {
            questions: vec![],
            input_text: String::from(""),
            client: client,
        }
    }
}

static EDITOR: Atom<String> = |_| String::from("");
// static FORMINPUT_KEY: Atom<String> = |_| String::from("forminput");
const FORMINPUT_KEY: &str = "forminput";

fn Editor(cx: Scope) -> Element {
    let editor_state = use_atom_state(&cx, EDITOR);
    let question_state = use_atom_state(&cx, APP);

    let send_input = move |content: String| {
        log::info!("Recieved input: {content}");
        // let question = parse_markdown_blocks(content.clone());
        let question = parse_markdown_v3(content.clone()).unwrap();
        log::info!("Questions: {question:#?}");

        question_state.modify(|curr| {
            AppState {
                questions: question,
                input_text: curr.input_text.clone(),
                client: curr.client.clone(),
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
                    log::info!("form event: {e:#?}");
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
    let question_state = use_atom_state(&cx, APP);
    let app_state = use_atom_state(&cx, APP);

    // let post_questions = move || {
    //     log::info!("Attempting to save questions...");
    //     log::info!("Questions save: {:?}", question_state);
    //     app_state
    //         .client
    //         .post("localhost:3000/survey")
    //         .body(question_state)
    //         .send()
    //         .await?;
    // };

    let post_questions = move |content, client: Client| {
        let id = nanoid_gen(12);

        cx.spawn(async move {
            log::info!("Attempting to save questions...");
            // log::info!("Questions save: {:?}", question_state);
            match client
                .post("http://localhost:3000/survey")
                .json(&CreateSurvey {
                    id,
                    plaintext: content,
                })
                // .header("Access-Control-Allow-Origin", "http://localhost:8080/")
                // .header("Access-Control-Allow-Origin", "http://localhost:3000/")
                // .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .await
            {
                Ok(x) => log::info!("success: {x:?}"),
                Err(x) => log::info!("error: {x:?}"),
            };
        })
    };

    cx.render(rsx! {
        button {
            prevent_default: "onclick",
            class: "hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2",
            onclick: move |evt| {
                log::info!("Pushed publish :)");
                post_questions("test".to_string(), app_state.client.clone());
                evt.cancel_bubble();
            },
            "Publish"
        }
    })
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        main{
            // class: "container mx-auto max-w-lg p-6",
            class: "container p-6",
            div{
                Editor {}
                Questions {}
            }

        }
    })
}

fn Questions(cx: Scope) -> Element {
    let app_state = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);

    cx.render(rsx! {
        form {
            prevent_default: "onclick",
            class: "",
            app_state.questions.iter().map(|q: &Question| rsx!{
                fieldset {
                    legend {
                        class: "text-base mt-5 font-medium text-gray-900",
                        "{q.text} - {q.qtype:?}"
                    }
                    {
                        q.options.iter().map(|option| {
                            let qtype = match q.qtype {
                                QuestionType::Radio => "radio",
                                QuestionType::Checkbox => "checkbox",
                                QuestionType::Text => "textarea",
                            };

                            rsx!{
                                div{
                                    key: "{option.id}",
                                    class: "flex items-center",
                                    input {
                                        id: "{option.id}",
                                        name: "{q.id}",
                                        r#type: "{qtype}",
                                        class: " m-3 border border-gray-400"
                                    }
                                    label {
                                        r#for: "{option.id}",
                                        class: " text-gray-700 font-medium",
                                        "{option.text}"
                                    }
                                }

                            }
                        })
                    }
                }
            })
        }
    })
}

fn app(cx: Scope) -> Element {
    let set_app = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);

    cx.render(rsx!(Router {
        Route { to: "", Home {}}
        Route { to: "/", Home {}}
        Redirect {from: "", to: "/"}
    }))
}

fn main() {
    // css
    // npx tailwindcss -i ./input.css -o ./public/output.css --watch

    // hot reload
    // cargo watch -- dioxus serve
    // kill hot reload
    // sudo lsof -i :8080
    // kill -9 PID

    /*

    1. question one
       1. option 1
       2. option 2
    2. testing


    - this is another
      - option 1 in other
    - test2 question
      - this is great
        */

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    // std::panic::set_hook(Box::new(|info| {
    //     println!("Panic: {}", info);
    // }));

    dioxus::web::launch_cfg(app, |c| c.into());
}
