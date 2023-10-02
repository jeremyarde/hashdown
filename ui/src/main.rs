use dioxus_web::Config;
use log::LevelFilter;
use ui::mainapp;

fn main() {
    // css
    // npx tailwindcss -i ./input.css -o ./public/output.css --watch
    // npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch

    // cargo watch -d 1 -- cargo run
    // kill hot reload
    // sudo lsof -i :8080
    // kill -9 PID

    // init debug tool for WebAssembly
    // wasm_logger::init(wasm_logger::Config::default());
    // console_error_panic_hook::set_once();
    // std::panic::set_hook(Box::new(|info| {
    //     println!("Panic: {}", info);
    // }));
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    // #[cfg(not(target_arch = "wasm32"))]
    // dioxus_desktop::launch_cfg(
    //     mainapp::App,
    //     dioxus_desktop::Config::new()
    //         .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string()),
    // );
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(mainapp::App);

    // #[cfg(not(target_arch = "wasm32"))]
    // dioxus_desktop::launch(mainapp::App);

    // dioxus_web::launch_cfg(mainapp::App, Config::new());
    // server_side()
    // dioxus::web::launch_cfg(app, |c| c.into());
}
