use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { class: "bg-red-600", "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales, I think this is all that is takes." }
            p { class: "bg-blue-600", "This is jeremy testing hot reload, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
    ))
}

fn main() {
    // cargo watch -- cargo run --package ui2 --bin ui2 --target wasm32-unknown-unknown
    // cargo watch -- cargo run --target wasm32-unknown-unknown

    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch_cfg(app, |c| c.into());
}
