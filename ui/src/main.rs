#![allow(non_snake_case)]
use dioxus::{
    events::{onclick, oninput},
    fermi::use_atom_state,
    prelude::{
        dioxus_elements::{h5, textarea, fieldset},
        *,
    },
};

// use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
use markdownparser::{parse_markdown_blocks, Question, Questions, parse_markdown_v3, QuestionType};

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
                div { class: "w-full mb-4 border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-700 dark:border-gray-600",
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
                    }
                }
            }
        }
    })
}


fn Questions(cx: Scope) -> Element {
    let app_state = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);



    cx.render(rsx! {
            div { 
                class: "mt-5 md:col-span-2 md:mt-0",
                form {
                    // action: "#",
                    // method: "POST",
                    div { 
                        class: "overflow-hidden shadow sm:rounded-md",
                        div {
                            class: "space-y-6 bg-white px-4 py-5 sm:p-6",
                            app_state.questions.iter().map(|q: &Question| rsx!{
                                fieldset {
                                    legend { 
                                        class: "sr-only",
                                        "{q.text}"
                                    }
                                    div { 
                                        aria_hidden: "true",
                                        class: "text-base font-medium text-gray-900",
                                        "{q.text}"
                                    }
                                    p {"{q.qtype:?}"}
                                    {
                                        // create_questions(q)
                                        q.options.iter().map(|option| {
                                            let qtype = match q.qtype {
                                                QuestionType::Radio => "radio",
                                                QuestionType::Checkbox => "checkbox",
                                                QuestionType::Text => "textarea",
                                            };

                                            rsx!{
                                                li {
                                                    class: "list-none mt-4 space-y-4  bg-gray-100 hover:bg-gray-200 space-x-2 flex items-start h-5",
                                                    key: "{option.id}",
                                                    input { 
                                                        id: "{option.id}",
                                                        r#type: "{qtype}",
                                                        class: "h-4 w-4 rounded border-gray-300 text-yellow-100-600 focus:fill-red-400",
                                                        name: "{q.id}",
                                                    }
                                                }
                                            }
                                        })
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    )
}


fn app(cx: Scope) -> Element {
    let set_app = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);

    cx.render(rsx! (
        main{
            class: "wrapper",
            div {
                style: "text-align: center; display: grid; grid-template-columns: 1f1 min(65ch, 100%) 1fr;",
                h1 { 
                    class: "bg-lime-200", 
                    "Write your survey questions in Markdown below" 
                }
            }
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

