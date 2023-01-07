#![allow(non_snake_case)]
use dioxus::{
    events::oninput,
    fermi::use_atom_state,
    prelude::{dioxus_elements::textarea, *},
};

// use fermi::{use_atom_ref, use_atom_state, use_set, Atom};
use markdownparser::{parse_markdown_blocks, Questions};

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
        }
    })
}

fn Questions(cx: Scope) -> Element {
    let app_state = use_atom_state(&cx, APP);
    let editor_state = use_atom_state(&cx, EDITOR);

    // let parse_markdown = move || {
    //     let questions = parse_markdown_blocks(editor_state.get().clone());
    // };

    cx.render(rsx! {
        div{
            ul{app_state.questions.iter().map(|f| rsx!{
                li {
                    div { class: "container m-auto grid bg-red-400"}
                    div{ class: "bg-green-200", "{f:?}"}
                }
            })}
            h2 { "tets h2"}
            h2 { "{editor_state}" }
            h3 { "testing" }
            h3 { "{app_state.questions:?}"}
        }
    })
}

fn app(cx: Scope) -> Element {
    let set_app = use_atom_state(&cx, APP);

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { class: "bg-red-200", "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales, I think this is all that is takes." }
            p { class: "bg-lime-600", "This is jeremy testing hot reload, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
        div{
            Editor {}
            Questions {}
            // h2 { "{set_app.input_text}" }
            // h3 { "{set_app.questions:?}"}
        }
    ))
}

fn main() {
    // cargo watch -- dioxus serve

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch_cfg(app, |c| c.into());
}
