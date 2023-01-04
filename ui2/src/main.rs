use dioxus::prelude::*;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch_cfg(app, |c| c.into());
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { class: "bg-gray-900", "🌗 Dioxus 🚀" }
            h3 { "Frontend that scales." }
            p { class: "bg-gray-300", "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
        }
    ))
}
