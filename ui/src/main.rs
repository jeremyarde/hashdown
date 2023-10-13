use dioxus_web::Config;
use log::LevelFilter;
use ui::mainapp::{self, AppState};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                BlogList {},
                #[route("/blog/:name")]
                BlogPost { name: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogList {})]
        #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}
// ANCHOR_END: router

#[component]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || AppState::new());

    render! { Router::<Route> {} }
}

#[component]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li {
                    Link { to: Route::Home {}, "Home" }
                }
                li {
                    Link { to: Route::BlogList {}, "Blog" }
                }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    render! { h1 { "Welcome to the Dioxus Blog!" } }
}

#[component]
fn Blog(cx: Scope) -> Element {
    render! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}

#[component]
fn BlogList(cx: Scope) -> Element {
    render! {
            h2 { "Choose a post" }
            ul {
                li {
                    Link {
                        to: Route::BlogPost {
        name: "Blog post 1".into(),
    },
                        "Read the first blog post"
                    }
                }
                li {
                    Link {
                        to: Route::BlogPost {
        name: "Blog post 2".into(),
    },
                        "Read the second blog post"
                    }
                }
            }
        }
}

#[component]
fn BlogPost(cx: Scope, name: String) -> Element {
    render! { h2 { "Blog Post: {name}" } }
}

#[component]
fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

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
    // #[cfg(target_arch = "wasm32")]
    // dioxus_web::launch(mainapp::App);

    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(App);

    // #[cfg(not(target_arch = "wasm32"))]
    // dioxus_desktop::launch(mainapp::App);

    // dioxus_web::launch_cfg(mainapp::App, Config::new());
    // server_side()
    // dioxus::web::launch_cfg(app, |c| c.into());
}
