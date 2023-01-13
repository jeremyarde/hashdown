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
    let test_data = r#" 
1. testing the first
    1. first option
    2. second option
2. second question
    1. last option
    "#.to_string();

    let test_data_element = create_test_data_rsx(cx, test_data);

    cx.render(rsx! (
        main{
            class: "wrapper",
            div {
                style: "text-align: center; display: grid; grid-template-columns: 1f1 min(65ch, 100%) 1fr;",
                p { class: "bg-lime-600", "This is jeremy testing hot reload, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
            }
            div{
                Editor {}
                Questions {}
            }
            div{
                test_data_element
            }
            div {
                component()
            }
            div {
                all_forms()
            }
        }
    ))
}



fn component(cx: Scope) -> Element {
    cx.render(rsx!(
        h1 { 
            "This is the dropdown comopnent"
        }
        div { 
            class: "col-span-6 sm:col-span-3",
            label { 
                class: "block text-sm font-medium text-gray-700",
                // htmlfor: "country",
                "Country"
            }
            select { 
                id: "country",
                name: "country",
                autocomplete: "country-name",
                class: "mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm",
                option { 
                    "United States"
                }
                option { 
                    "Canada"
                }
                option { 
                    "Mexico"
                }
            }
        }
    ))
}

fn create_test_data_rsx(cx: Scope, string: String) -> Element {
    return cx.render(rsx!{
        string.lines().into_iter().map(|line| rsx!{p{"{line}"}})
    })
}


fn all_forms(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
        }
        div { 
            aria_hidden: "true",
            class: "hidden sm:block",
            div { 
                class: "py-5",
                div { 
                    class: "border-t border-gray-200",
                }
            }
        }
        div { 
            class: "mt-10 sm:mt-0",
            div { 
                class: "md:grid md:grid-cols-3 md:gap-6",
                div { 
                    class: "md:col-span-1",
                    div { 
                        class: "px-4 sm:px-0",
                        h3 { 
                            class: "text-lg font-medium leading-6 text-gray-900",
                            "Personal Information"
                        }
                        p { 
                            class: "mt-1 text-sm text-gray-600",
                            "Use a permanent address where you can receive mail."
                        }
                    }
                }
                div { 
                    class: "mt-5 md:col-span-2 md:mt-0",
                    form { 
                        method: "POST",
                        action: "#",
                        div { 
                            class: "overflow-hidden shadow sm:rounded-md",
                            div { 
                                class: "bg-white px-4 py-5 sm:p-6",
                                div { 
                                    class: "grid grid-cols-6 gap-6",
                                    div { 
                                        class: "col-span-6 sm:col-span-3",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "First name"
                                        }
                                        input { 
                                            id: "first-name",
                                            name: "first-name",
                                            r#type: "text",
                                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            autocomplete: "given-name",
                                        }
                                    }
                                    div { 
                                        class: "col-span-6 sm:col-span-3",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "Last name"
                                        }
                                        input { 
                                            id: "last-name",
                                            autocomplete: "family-name",
                                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            name: "last-name",
                                            r#type: "text",
                                        }
                                    }
                                    div { 
                                        class: "col-span-6 sm:col-span-4",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "Email address"
                                        }
                                        input { 
                                            id: "email-address",
                                            name: "email-address",
                                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            r#type: "text",
                                            autocomplete: "email",
                                        }
                                    }
                                    div { 
                                        class: "col-span-6 sm:col-span-3",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "Country"
                                        }
                                        select { 
                                            id: "country",
                                            name: "country",
                                            class: "mt-1 block w-full rounded-md border border-gray-300 bg-white py-2 px-3 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm",
                                            autocomplete: "country-name",
                                            option { 
                                                "United States"
                                            }
                                            option { 
                                                "Canada"
                                            }
                                            option { 
                                                "Mexico"
                                            }
                                        }
                                    }
                                    div { 
                                        class: "col-span-6",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "Street address"
                                        }
                                        input { 
                                            id: "street-address",
                                            r#type: "text",
                                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            name: "street-address",
                                            autocomplete: "street-address",
                                        }
                                    }
                                    div { 
                                        class: "col-span-6 sm:col-span-6 lg:col-span-2",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "City"
                                        }
                                        input { 
                                            id: "city",
                                            r#type: "text",
                                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            autocomplete: "address-level2",
                                            name: "city",
                                        }
                                    }
                                    div { 
                                        class: "col-span-6 sm:col-span-3 lg:col-span-2",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "State / Province"
                                        }
                                        input { 
                                            id: "region",
                                            autocomplete: "address-level1",
                                            r#type: "text",
                                            name: "region",
                                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                        }
                                    }
                                    div { 
                                        class: "col-span-6 sm:col-span-3 lg:col-span-2",
                                        label { 
                                            class: "block text-sm font-medium text-gray-700",
                                            "ZIP / Postal code"
                                        }
                                        input { 
                                            id: "postal-code",
                                            autocomplete: "postal-code",
                                            r#type: "text",
                                            class: "mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm",
                                            name: "postal-code",
                                        }
                                    }
                                }
                            }
                            div { 
                                class: "bg-gray-50 px-4 py-3 text-right sm:px-6",
                                button { 
                                    r#type: "submit",
                                    class: "inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                                    "Save"
                                }
                            }
                        }
                    }
                }
            }
        }
        div { 
            class: "hidden sm:block",
            aria_hidden: "true",
            div { 
                class: "py-5",
                div { 
                    class: "border-t border-gray-200",
                }
            }
        }
        div { 
            class: "mt-10 sm:mt-0",
            div { 
                class: "md:grid md:grid-cols-3 md:gap-6",
                div { 
                    class: "md:col-span-1",
                    div { 
                        class: "px-4 sm:px-0",
                        h3 { 
                            class: "text-lg font-medium leading-6 text-gray-900",
                            "Notifications"
                        }
                        p { 
                            class: "mt-1 text-sm text-gray-600",
                            "Decide which communications you'd like to receive and how."
                        }
                    }
                }
                div { 
                    class: "mt-5 md:col-span-2 md:mt-0",
                    form { 
                        action: "#",
                        method: "POST",
                        div { 
                            class: "overflow-hidden shadow sm:rounded-md",
                            div { 
                                class: "space-y-6 bg-white px-4 py-5 sm:p-6",
                                fieldset { 
                                    legend { 
                                        class: "sr-only",
                                        "By Email"
                                    }
                                    div { 
                                        aria_hidden: "true",
                                        class: "text-base font-medium text-gray-900",
                                        "By Email"
                                    }
                                    div { 
                                        class: "mt-4 space-y-4",
                                        div { 
                                            class: "flex items-start",
                                            div { 
                                                class: "flex h-5 items-center",
                                                input { 
                                                    id: "comments",
                                                    r#type: "checkbox",
                                                    class: "h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500",
                                                    name: "comments",
                                                }
                                            }
                                            div { 
                                                class: "ml-3 text-sm",
                                                label { 
                                                    class: "font-medium text-gray-700",
                                                    "Comments"
                                                }
                                                p { 
                                                    class: "text-gray-500",
                                                    "Get notified when someones posts a comment on a posting."
                                                }
                                            }
                                        }
                                        div { 
                                            class: "flex items-start",
                                            div { 
                                                class: "flex h-5 items-center",
                                                input { 
                                                    id: "candidates",
                                                    class: "h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500",
                                                    r#type: "checkbox",
                                                    name: "candidates",
                                                }
                                            }
                                            div { 
                                                class: "ml-3 text-sm",
                                                label { 
                                                    class: "font-medium text-gray-700",
                                                    "Candidates"
                                                }
                                                p { 
                                                    class: "text-gray-500",
                                                    "Get notified when a candidate applies for a job."
                                                }
                                            }
                                        }
                                        div { 
                                            class: "flex items-start",
                                            div { 
                                                class: "flex h-5 items-center",
                                                input { 
                                                    id: "offers",
                                                    name: "offers",
                                                    r#type: "checkbox",
                                                    class: "h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-500",
                                                }
                                            }
                                            div { 
                                                class: "ml-3 text-sm",
                                                label { 
                                                    class: "font-medium text-gray-700",
                                                    "Offers"
                                                }
                                                p { 
                                                    class: "text-gray-500",
                                                    "Get notified when a candidate accepts or rejects an offer."
                                                }
                                            }
                                        }
                                    }
                                }
                                fieldset { 
                                    legend { 
                                        class: "contents text-base font-medium text-gray-900",
                                        "Push Notifications"
                                    }
                                    p { 
                                        class: "text-sm text-gray-500",
                                        "These are delivered via SMS to your mobile phone."
                                    }
                                    div { 
                                        class: "mt-4 space-y-4",
                                        div { 
                                            class: "flex items-center",
                                            input { 
                                                id: "push-everything",
                                                r#type: "radio",
                                                class: "h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500",
                                                name: "push-notifications",
                                            }
                                            label { 
                                                class: "ml-3 block text-sm font-medium text-gray-700",
                                                "Everything"
                                            }
                                        }
                                        div { 
                                            class: "flex items-center",
                                            input { 
                                                id: "push-email",
                                                class: "h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500",
                                                r#type: "radio",
                                                name: "push-notifications",
                                            }
                                            label { 
                                                class: "ml-3 block text-sm font-medium text-gray-700",
                                                "Same as email"
                                            }
                                        }
                                        div { 
                                            class: "flex items-center",
                                            input { 
                                                id: "push-nothing",
                                                r#type: "radio",
                                                name: "push-notifications",
                                                class: "h-4 w-4 border-gray-300 text-indigo-600 focus:ring-indigo-500",
                                            }
                                            label { 
                                                class: "ml-3 block text-sm font-medium text-gray-700",
                                                "No push notifications"
                                            }
                                        }
                                    }
                                }
                            }
                            div { 
                                class: "bg-gray-50 px-4 py-3 text-right sm:px-6",
                                button { 
                                    class: "inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                                    r#type: "submit",
                                    "Save"
                                }
                            }
                        }
                    }
                }
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

