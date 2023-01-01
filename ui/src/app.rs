use leptos::{web_sys::Event, *};
use leptos_meta::*;
use markdownparser::Question;

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());
    let (count, set_count) = create_signal(cx, 0);
    let (content, set_content) = create_signal(cx, "");
    let (questions, set_questions) = create_signal(cx, vec![]);
    let content_input = NodeRef::new(cx);
    // let mut questions: Vec<Question> = vec![];

    let mut handle_input_change = move |ev| {
        let string_content = content_input
            .get()
            .expect("input should contain something")
            .unchecked_into::<web_sys::HtmlInputElement>()
            .value();
        log!("content: {:?}", string_content);
        set_questions.update(|x| *x = markdownparser::parse_markdown_blocks(string_content));
        log!("parse results: {:?}", questions());
    };

    view! {
        cx,
        <main class="my-0 mx-auto max-w-3xl text-center">
        <Stylesheet id="leptos" href="/style.css"/>
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| {log!("add one, val: {count}");*count += 1;})
            >
            {"Click me"}
            </button>
            {count}
            <div>
            <h2>{"TODO"}</h2>
            <textarea
                type="text"         // attributes work just like they do in HTML
                name="new-todo"
                prop:value="todo"   // `prop:` lets you set a property on a DOM node
                value="initial"     // side note: the DOM `value` attribute only sets *initial* value
                                    // this is very important when working with forms!
                _ref=content_input // `_ref` stores tis element in a variable
                on:input=move |ev| handle_input_change(ev)
            />
        // <input
        //     type="text"         // attributes work just like they do in HTML
        //     name="new-todo"
        //     prop:value="todo"   // `prop:` lets you set a property on a DOM node
        //     value="initial"     // side note: the DOM `value` attribute only sets *initial* value
        //                         // this is very important when working with forms!
        //     _ref=content_input // `_ref` stores tis element in a variable
        //     on:input=move |ev| handle_input_change(ev)
        // />
        {questions().iter().map(move |question: &Question| view! {cx, <h3>{&question.text}</h3>}).collect::<Vec<_>>()}

        </div>
        </main>
    }
}
