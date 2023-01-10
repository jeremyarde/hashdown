#![allow(non_snake_case)]
use dioxus::{
    events::{onclick, oninput},
    fermi::use_atom_state,
    prelude::{
        dioxus_elements::{h5, textarea},
        *,
    },
};

// use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
use markdownparser::{parse_markdown_blocks, Question, Questions};

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
        let question = parse_markdown_blocks(content.clone());
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
            textarea { rows: "10", cols: "50", oninput: move |e| {send_input(e.value.clone())}}
            button { 
                class:"px-5 py-5 font-bold text-white bg-blue-500 rounded hover:bg-blue-700", 
                onclick: move |_| send_input("1. this is a question\\n  1. option1\\n  2. option 2 here".to_string()), 
                "Debug"
            }
        }
    })
}

fn Questions(cx: Scope) -> Element {
    let app_state = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);

    cx.render(rsx! {
        div{
            ul{app_state.questions.iter().map(|q| rsx!{
                li {
                    div { class: "container m-auto grid bg-red-400",
                        h5 { "{q.text}"}
                        ol {
                            q.options.iter().map(|o| rsx!{li {"{o}"}})
                        }
                    }
                    // div{ class: "bg-green-200", "{q:?}"}
                }
            })}
            h2 { "{editor_state}" }
            h3 { "{app_state.questions:?}"}
        }
    })
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
                    style: "color: red; background-color: green;",
                    "test regular css in html" }
                h1 { class: "bg-red-200", "🌗 Dioxus 🚀" }
                h3 { "Frontend that scales, I think this is all that is takes." }
                p { class: "bg-lime-600", "This is jeremy testing hot reload, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
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
    // 

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch_cfg(app, |c| c.into());
}
