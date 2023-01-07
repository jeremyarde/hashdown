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

impl AppState {
    fn new() -> Self {
        AppState {
            questions: vec![],
            input_text: String::from(""),
        }
    }
}

fn app(cx: Scope) -> Element {
    // let model = use_state(&cx, || String::from(""));
    // let results = use_state(&cx, || String::from(""));
    // let set_app = use_set(&cx, APP);
    let set_app = use_atom_state(&cx, APP);

    let send_input = move |content: String| {
        print!("Testing in send inputa");
        let question = parse_markdown_blocks(content.clone());
        // println!("results: {:?}", results);
        // set_app = question;
        let _x = &set_app.get().questions;
        set_app.set(AppState {
            questions: question,
            input_text: content.clone(),
        });
    };

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { class: "bg-red-600", "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales, I think this is all that is takes." }
            p { class: "bg-blue-600", "This is jeremy testing hot reload, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
        div{
            textarea { rows: "10", cols: "100", oninput: move |e| {send_input(e.value.clone())}}
            h2 { "{set_app.input_text}" }
            h3 { "{set_app.questions:?}"}
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
