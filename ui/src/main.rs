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

fn Editor(cx: Scope) -> Element {
    let editor_state = use_atom_state(&cx, EDITOR);
    let question_state = use_atom_state(&cx, APP);

    let send_input = move |content: String| {
        print!("Testing in send inputa");
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

    cx.render(rsx! {
        div{
            form {
                // textarea { 
                //     rows: "10", cols: "50", oninput: move |e| {send_input(e.value.clone())},
                //     class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                //     placeholder: "Write your thoughts here..."
                // }
                form { 
                    div { class: "w-full mb-4 border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-700 dark:border-gray-600",
                        // div { class: "flex items-center justify-between px-3 py-2 border-b dark:border-gray-600",
                        // }
                        div { class: "px-4 py-2 bg-white rounded-b-lg dark:bg-gray-800",
                            label { class: "sr-only",
                                r#for: "editor",
                                "Publish post"
                            }
                            textarea { class: "block w-full px-0 text-sm text-gray-800 bg-white border-0 dark:bg-gray-800 focus:ring-0 dark:text-white dark:placeholder-gray-400",
                                id: "editor",
                                required: "",
                                rows: "8",
                                placeholder: "Write your survey here",
                                oninput: move |e| {send_input(e.value.clone())},
                                ""
                            }
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
                                                div { 
                                                    class: "mt-4 space-y-4",
                                                    div { 
                                                        class: "flex items-start",
                                                        div { 
                                                            class: "flex h-5 items-center",
                                                            input { 
                                                                id: "comments",
                                                                r#type: "{qtype}",
                                                                class: "h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500",
                                                                name: "comments",
                                                            }
                                                        }
                                                        div { 
                                                            class: "ml-3 text-sm",
                                                            label { 
                                                                class: "font-medium text-gray-700",
                                                                "{option}"
                                                            }
                                                        }
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

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch_cfg(app, |c| c.into());
}

