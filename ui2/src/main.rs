use dioxus::{
    events::oninput,
    prelude::{dioxus_elements::textarea, *},
};

use markdownparser::parse_markdown_blocks;

fn app(cx: Scope) -> Element {
    let model = use_state(&cx, || String::from(""));

    fn send_input(content: String) {
        print!("Testing in send input");
        parse_markdown_blocks(content);
    }

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { class: "bg-red-600", "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales, I think this is all that is takes." }
            p { class: "bg-blue-600", "This is jeremy testing hot reload, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
        div{
            textarea { rows: "10", cols: "100", oninput: move |e| {model.set(e.value.clone());send_input(e.value.clone());}}
        }
    ))
}

fn main() {
    // cargo watch -- cargo run --package ui2 --bin ui2 --target wasm32-unknown-unknown
    // cargo watch -- cargo run --target wasm32-unknown-unknown

    // cargo 

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch_cfg(app, |c| c.into());
}
