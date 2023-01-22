#![allow(non_snake_case)]
use dioxus::{
    events::{onclick, oninput},
    fermi::use_atom_state,
    prelude::{
        dioxus_elements::{button, fieldset, h5, textarea},
        *,
    },
};

// use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
use markdownparser::{parse_markdown_blocks, parse_markdown_v3, Question, QuestionType, Questions};

static APP: Atom<AppState> = |_| AppState::new();

struct AppState {
    questions: Questions,
    input_text: String,
}

struct Survey {
    title: String,
    questions: Questions,
}

impl AppState {
    fn new() -> Self {
        AppState {
            questions: vec![],
            input_text: String::from(""),
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
            }
            // curr.questions = question;
        });
        // let _x = &set_app.get().questions;
        editor_state.set(content);
    };

    // send_input("1. testing\n 1. another\n 2. second option".to_string());

    cx.render(rsx! {
        div{
            form {
                oninput: move |e| {
                    log::info!("form event: {e:#?}");
                    let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                    send_input(formvalue);
                },
                div { class: "w-full m-4  border-gray-200 bg-gray-50 dark:bg-gray-700 dark:border-gray-600",
                    // div { class: "flex items-center justify-between px-3 py-2 border-b dark:border-gray-600",
                    // }
                    div { class: "px-4 py-2 bg-white rounded-b-lg dark:bg-gray-800 focus:ring-red-500",
                        id: "editor",
                        label { class: "sr-only",
                            r#for: "editor",
                            "Publish post"
                        }
                        textarea { class: "block w-full px-0 text-sm text-gray-800 bg-white border-0 dark:bg-gray-800   dark:text-white dark:placeholder-gray-400",
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
        }
    })
}

fn Publish(cx: Scope) -> Element {
    cx.render(rsx! {
        button {
            class: "hover:bg-violet-600 w-full text-blue-500 bg-blue-200 rounded p-2",
            onclick: move |_| {log::info!("Pushed publish :)")},
            "Publish"
        }
    })
}

fn Questions(cx: Scope) -> Element {
    let app_state = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);

    cx.render(rsx! {
        div {
            class: "",
            form {
                // action: "#",
                // method: "POST",
                div {
                    class: "shadow bg-yellow-100",
                        app_state.questions.iter().map(|q: &Question| rsx!{
                            legend {
                                class: "sr-only",
                                "{q.text}"
                            }
                            div {
                                aria_hidden: "true",
                                class: "text-base font-medium text-gray-900",
                                "{q.text} - {q.qtype:?}"
                            }
                            {
                                // create_questions(q)
                                q.options.iter().map(|option| {
                                    let qtype = match q.qtype {
                                        QuestionType::Radio => "radio",
                                        QuestionType::Checkbox => "checkbox",
                                        QuestionType::Text => "textarea",
                                    };

                                    rsx!{
                                        // li {
                                        //     class: "list-none mt-4 space-y-4  bg-gray-100 hover:bg-gray-200 space-x-2 flex items-start h-5",
                                        //     key: "{option.id}",
                                        //     input {
                                        //         id: "{option.id}",
                                        //         r#type: "{qtype}",
                                        //         class: "h-4 w-4 rounded border-gray-300 text-yellow-100-600 focus:fill-red-400",
                                        //         name: "{q.id}",
                                        //     }
                                        // }
                                        div{
                                            key: "{option.id}",
                                            class: "bg-blue-200 focus:fill-red-400",
                                            input {
                                                id: "{option.id}",
                                                name: "{q.id}",
                                                r#type: "{qtype}",
                                                class: "bg-gray-100"
                                            }
                                            label {
                                                r#for: "{option.id}",
                                                class: "text-gray-900",
                                                "{option.text}"
                                            }
                                        }

                                    }
                                })
                            }
                        })
                }
            }
        }
    })
}

fn app(cx: Scope) -> Element {
    let set_app = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);

    cx.render(rsx! (
        main{
            div{
                Editor {}
                Questions {}
            }

        }
    ))
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

    dioxus::web::launch_cfg(app, |c| c.into());
}
