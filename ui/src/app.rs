use leptos::{
    web_sys::{Event, Node},
    *,
};
use leptos_meta::*;

use markdownparser::{Question, Questions};

#[derive(Copy, Clone)]
struct QuestionsContext(ReadSignal<Questions>);
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // provide_meta_context(cx);

    let (count, set_count) = create_signal(cx, 0);

    view! {
        cx,
        <main class="my-0 mx-auto max-w-3xl text-center">
        <Stylesheet href="/style.css"/>
            <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
            <p class="px-10 pb-10 text-left">"Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."</p>
            <button
                class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
                on:click=move |_| set_count.update(|count| *count += 1)
            >
                {move || if count() == 0 {
                    "Click me!".to_string()
                } else {
                    count().to_string()
                }}
            </button>
        </main>
    }
}

#[component]
fn QuestionsComponent(cx: Scope) -> impl IntoView {
    let (questions, set_questions) = create_signal(cx, vec![]);
    let content_input: NodeRef<&str> = NodeRef::new(cx);
    // let sample_q = Question {
    //     id: 0,
    //     text: String::from("tests"),
    //     options: vec![],
    // };
    provide_context(cx, QuestionsContext(questions));
    // let mut questions: Vec<Question> = vec![];

    let mut handle_input_change = move |ev: Event| {
        let string_content = content_input.get().expect("input should contain something");
        // .unchecked_into::<web_sys::HtmlInputElement>()
        // .value();
        log!("content: {:?}", string_content);
        let results = markdownparser::parse_markdown_blocks(string_content.to_string());
        // set_questions.update(|x| {
        //     x.clear();
        //     // x.extend(results);
        // });

        log!("parse results: {:?}", questions.get());
    };

    let questions = use_context::<QuestionsContext>(cx).unwrap().0;
    log!("questions - {:?}", questions.get());
    let temp = Question {
        id: 0,
        text: String::from("tests"),
        options: vec![],
    };

    // let first_q = questions.get().get(0).unwrap_or(&temp);

    let question_list = move || {
        questions
            .get()
            .into_iter()
            .map(|x| view! {cx, <li>{x.text}</li>})
            .collect::<Vec<_>>()
    };
    // log!("firs question: {:?}", &first_q);

    // view! {
    //     cx,
    //     <div>
    //     <h2>"This is the questions section"</h2>
    //     <textarea
    //         type="text"         // attributes work just like they do in HTML
    //         name="new-todo"
    //         prop:value=""
    //         value="initial"     // side note: the DOM `value` attribute only sets *initial* value
    //                             // this is very important when working with forms!
    //         _ref=content_input // `_ref` stores tis element in a variable
    //         on:input=move |ev| handle_input_change(ev)
    //     />
    //     // <p>{questions.iter().map(|x: Question| {view! {x.text}})}</p>
    //     <ul>
    //         {question_list}
    //     </ul>

    //     </div>
    // }
}

// #[component]
// fn SingleQuestion(cx: Scope) -> Element {
//     let question = Question {
//         id: 0,
//         text: String::from("sample q"),
//         options: vec![],
//     };
//     view! {
//         cx, <li>
//         <div>
//         <h3>{question.text}</h3>
//         </div></li>
//     }
// }
