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
            form {
                // textarea { 
                //     rows: "10", cols: "50", oninput: move |e| {send_input(e.value.clone())},
                //     class: "block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                //     placeholder: "Write your thoughts here..."
                // }
                form { 
                    div { class: "w-full mb-4 border border-gray-200 rounded-lg bg-gray-50 dark:bg-gray-700 dark:border-gray-600",
                        div { class: "flex items-center justify-between px-3 py-2 border-b dark:border-gray-600",
                            div { class: "flex flex-wrap items-center divide-gray-200 sm:divide-x dark:divide-gray-600",
                                div { class: "flex items-center space-x-1 sm:pr-4",
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            view_box: "0 0 20 20",
                                            // aria_hidden: "true",
                                            fill: "currentColor",
                                            path { 
                                                d: "M8 4a3 3 0 00-3 3v4a5 5 0 0010 0V7a1 1 0 112 0v4a7 7 0 11-14 0V7a5 5 0 0110 0v4a3 3 0 11-6 0V7a1 1 0 012 0v4a1 1 0 102 0V7a3 3 0 00-3-3z",
                                                clip_rule: "evenodd",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Attach file"
                                        }
                                    }
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            fill: "currentColor",
                                            // aria_hidden: "true",
                                            view_box: "0 0 20 20",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            path { 
                                                fill_rule: "evenodd",
                                                d: "M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Embed map"
                                        }
                                    }
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "currentColor",
                                            // aria_hidden: "true",
                                            view_box: "0 0 20 20",
                                            path { 
                                                fill_rule: "evenodd",
                                                clip_rule: "evenodd",
                                                d: "M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Upload image"
                                        }
                                    }
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            // aria_hidden: "true",
                                            path { 
                                                clip_rule: "evenodd",
                                                fill_rule: "evenodd",
                                                d: "M12.316 3.051a1 1 0 01.633 1.265l-4 12a1 1 0 11-1.898-.632l4-12a1 1 0 011.265-.633zM5.707 6.293a1 1 0 010 1.414L3.414 10l2.293 2.293a1 1 0 11-1.414 1.414l-3-3a1 1 0 010-1.414l3-3a1 1 0 011.414 0zm8.586 0a1 1 0 011.414 0l3 3a1 1 0 010 1.414l-3 3a1 1 0 11-1.414-1.414L16.586 10l-2.293-2.293a1 1 0 010-1.414z",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Format code"
                                        }
                                    }
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            // aria_hidden: "true",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            path { 
                                                fill_rule: "evenodd",
                                                d: "M10 18a8 8 0 100-16 8 8 0 000 16zM7 9a1 1 0 100-2 1 1 0 000 2zm7-1a1 1 0 11-2 0 1 1 0 012 0zm-.464 5.535a1 1 0 10-1.415-1.414 3 3 0 01-4.242 0 1 1 0 00-1.415 1.414 5 5 0 007.072 0z",
                                                clip_rule: "evenodd",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Add emoji"
                                        }
                                    }
                                }
                                div { class: "flex flex-wrap items-center space-x-1 sm:pl-4",
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            // aria_hidden: "true",
                                            view_box: "0 0 20 20",
                                            fill: "currentColor",
                                            path { 
                                                d: "M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z",
                                                clip_rule: "evenodd",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Add list"
                                        }
                                    }
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            view_box: "0 0 20 20",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            // aria_hidden: "true",
                                            fill: "currentColor",
                                            path { 
                                                clip_rule: "evenodd",
                                                fill_rule: "evenodd",
                                                d: "M11.49 3.17c-.38-1.56-2.6-1.56-2.98 0a1.532 1.532 0 01-2.286.948c-1.372-.836-2.942.734-2.106 2.106.54.886.061 2.042-.947 2.287-1.561.379-1.561 2.6 0 2.978a1.532 1.532 0 01.947 2.287c-.836 1.372.734 2.942 2.106 2.106a1.532 1.532 0 012.287.947c.379 1.561 2.6 1.561 2.978 0a1.533 1.533 0 012.287-.947c1.372.836 2.942-.734 2.106-2.106a1.533 1.533 0 01.947-2.287c1.561-.379 1.561-2.6 0-2.978a1.532 1.532 0 01-.947-2.287c.836-1.372-.734-2.942-2.106-2.106a1.532 1.532 0 01-2.287-.947zM10 13a3 3 0 100-6 3 3 0 000 6z",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Settings"
                                        }
                                    }
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            fill: "currentColor",
                                            view_box: "0 0 20 20",
                                            // aria_hidden: "true",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            path { 
                                                d: "M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z",
                                                clip_rule: "evenodd",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Timeline"
                                        }
                                    }
                                    button { class: "p-2 text-gray-500 rounded cursor-pointer hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                        r#type: "button",
                                        svg { class: "w-5 h-5",
                                            fill: "currentColor",
                                            // aria_hidden: "true",
                                            view_box: "0 0 20 20",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            path { 
                                                d: "M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z",
                                                clip_rule: "evenodd",
                                                fill_rule: "evenodd",
                                            }
                                        }
                                        span { class: "sr-only",
                                            "Download"
                                        }
                                    }
                                }
                            }
                            button { class: "p-2 text-gray-500 rounded cursor-pointer sm:ml-auto hover:text-gray-900 hover:bg-gray-100 dark:text-gray-400 dark:hover:text-white dark:hover:bg-gray-600",
                                // data_tooltip_target: "tooltip-fullscreen",
                                r#type: "button",
                                svg { class: "w-5 h-5",
                                    // aria_hidden: "true",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "currentColor",
                                    view_box: "0 0 20 20",
                                    path { 
                                        clip_rule: "evenodd",
                                        d: "M3 4a1 1 0 011-1h4a1 1 0 010 2H6.414l2.293 2.293a1 1 0 11-1.414 1.414L5 6.414V8a1 1 0 01-2 0V4zm9 1a1 1 0 010-2h4a1 1 0 011 1v4a1 1 0 01-2 0V6.414l-2.293 2.293a1 1 0 11-1.414-1.414L13.586 5H12zm-9 7a1 1 0 012 0v1.586l2.293-2.293a1 1 0 111.414 1.414L6.414 15H8a1 1 0 010 2H4a1 1 0 01-1-1v-4zm13-1a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 010-2h1.586l-2.293-2.293a1 1 0 111.414-1.414L15 13.586V12a1 1 0 011-1z",
                                        fill_rule: "evenodd",
                                    }
                                }
                                span { class: "sr-only",
                                    "Full screen"
                                }
                            }
                            div { class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white transition-opacity duration-300 bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-700",
                                id: "tooltip-fullscreen",
                                role: "tooltip",
                                "Show full screen"
                                div { class: "tooltip-arrow",
                                    // data_popper_arrow: "",
                                }
                            }
                        }
                        div { class: "px-4 py-2 bg-white rounded-b-lg dark:bg-gray-800",
                            label { class: "sr-only",
                                r#for: "editor",
                                "Publish post"
                            }
                            textarea { class: "block w-full px-0 text-sm text-gray-800 bg-white border-0 dark:bg-gray-800 focus:ring-0 dark:text-white dark:placeholder-gray-400",
                                id: "editor",
                                required: "",
                                rows: "8",
                                placeholder: "Write an article...",
                                oninput: move |e| {send_input(e.value.clone())},
                                ""
                            }
                        }
                    }
                    button { class: "inline-flex items-center px-5 py-2.5 text-sm font-medium text-center text-white bg-blue-700 rounded-lg focus:ring-4 focus:ring-blue-200 dark:focus:ring-blue-900 hover:bg-blue-800",
                        r#type: "submit",
                        "Publish post"
                    }
                }
                

            }
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
                h1 { 
                    class: "bg-lime-200", 
                    "Write your survey questions in Markdown below" 
                }
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

